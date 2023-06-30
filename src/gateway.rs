use crate::db::Database;
use crate::errors::GatewayErrors;
use crate::utils::{compact_y_parity_and_s, decode_dns_name};
use crate::ResolverCalls;
use async_trait::async_trait;
use ccip_read_server::types::{CCIPReadHandler, RPCCall};
use ccip_read_server::Server;
use ethers::abi::{encode, AbiDecode, AbiEncode};
use ethers::abi::{AbiParser, Token};
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::signers::Wallet;
use ethers::types::{Address, H160, U256, U64};
use ethers::utils::{hex, keccak256};
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::Arc;
use tracing::{debug, error, info};

#[derive(Clone)]
pub struct Gateway {
    signer: Wallet<SigningKey>,
    server: Server,
    ttl: u64,
    db: Arc<dyn Database + Sync + Send>,
}

impl Gateway {
    pub async fn new(
        signer: Wallet<SigningKey>,
        ttl: u64,
        ip_address: IpAddr,
        port: u16,
        db: Arc<dyn Database + Sync + Send>,
    ) -> Result<Self, GatewayErrors> {
        let server = Server::new(ip_address, port);
        Ok(Gateway {
            signer,
            server,
            ttl,
            db,
        })
    }

    pub async fn start(mut self) -> Result<(), GatewayErrors> {
        // Offchain resolver contract ABI
        let abi = AbiParser::default()
            .parse_str(r#"[function resolve(bytes memory name, bytes memory data) external view returns(bytes result, uint64 expires, bytes sig)]"#)?;

        self.server.add(abi, "resolve", Arc::new(self.clone()))?;

        Ok(self.server.start(None).await?)
    }

    #[tracing::instrument(
        name = "ens_query"
        skip_all
    )]
    async fn query(
        &self,
        db: Arc<dyn Database + Send + Sync>,
        domain_name: &str,
        data: &Vec<u8>,
    ) -> Result<(Vec<u8>, u64), GatewayErrors> {
        let name: ResolverCalls = ResolverCalls::decode(data)?;
        let result: Option<Token> = match name {
            ResolverCalls::Addr(_) => {
                debug!("Addr with coin type ETH");
                db.addr(domain_name)
                    .await
                    .map(|record| Token::Address(record.addr))
                    .or(Some(Token::Address(Address::zero())))
            }
            ResolverCalls::AddrWithCoinType(call) => {
                debug!("Addr with coin type {:?}", call.coin_type);
                db.addr_coin_type(domain_name, call.coin_type)
                    .await
                    .map(|record| Token::Bytes(record.addr.to_vec()))
                    .or(Some(Token::Bytes(vec![])))
            }
            ResolverCalls::Text(call) => {
                debug!("Text with call: {:?}", call);
                db.text(domain_name, &call.key)
                    .await
                    .map(|record| Token::String(record.value))
                    .or(Some(Token::String("".to_string())))
            }
            ResolverCalls::Contenthash(_) => {
                debug!("Content hash");
                db.contenthash(domain_name)
                    .await
                    .map(|record| {
                        Token::Bytes(hex::decode(record.content_hash.as_bytes()).unwrap())
                    })
                    .or(Some(Token::Bytes(vec![])))
            }
            ResolverCalls::Abi(_) => {
                debug!("ABI");
                Some(Token::Tuple(vec![
                    Token::Uint(U256::zero()),
                    Token::Bytes(vec![]),
                ]))
            }
            ResolverCalls::Pubkey(_) => {
                debug!("Pubkey");
                Some(Token::Tuple(vec![
                    Token::FixedBytes(vec![0]),
                    Token::FixedBytes(vec![0]),
                ]))
            }
            _ => {
                error!("Unsupported call type {:?}", name);
                None
            }
        };
        let result = result.ok_or(GatewayErrors::UnknownRecordType)?;

        info!("Record data: {:?}", &result);
        let result = encode(&[result]);

        let now = chrono::offset::Utc::now();

        Ok((result, now.timestamp() as u64 + self.ttl))
    }
}

#[async_trait]
impl CCIPReadHandler for Gateway {
    async fn call(
        &self,
        args: Vec<Token>,
        req: RPCCall,
    ) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
        let encoded_name = &args[0];
        let data = &args[1];
        let data = data
            .clone()
            .into_bytes()
            .ok_or(GatewayErrors::InvalidName)?;
        let name = decode_dns_name(encoded_name)?;

        // query the database
        let (result, valid_until) = self.query(self.db.clone(), &name, &data).await?;

        // Hash and sign the response
        let encoded = ethers::abi::encode_packed(&[
            Token::Uint(U256::from(0x1900)),
            Token::Address(H160::from_str(req.to.as_str())?),
            Token::FixedBytes(U64::from(valid_until).0[0].to_be_bytes().to_vec()),
            Token::FixedBytes(keccak256(&req.data).to_vec()),
            Token::FixedBytes(keccak256(&result).to_vec()),
        ])?;
        let message_hash = keccak256(encoded);

        let signature = self.signer.sign_hash(message_hash.into())?;
        let y_parity_and_s = compact_y_parity_and_s(&signature)?;
        let signature_encoded = [signature.r.encode(), y_parity_and_s].concat();

        Ok(vec![
            Token::Bytes(result),
            Token::Uint(U256::from(valid_until)),
            Token::Bytes(signature_encoded),
        ])
    }
}
