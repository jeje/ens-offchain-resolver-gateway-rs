use thiserror::Error;

#[derive(Error, Debug)]
pub enum GatewayErrors {
    #[error("invalid dns name")]
    InvalidName,
    #[error("ABI error {0}")]
    Abi(#[from] ethers::contract::AbiError),
    #[error("ABI parse error {0}")]
    AbiParse(#[from] ethers::abi::ParseError),
    #[error("CCIP Read error {0}")]
    CCIPReadMiddleware(#[from] ccip_read_server::CCIPReadMiddlewareError),
    #[error("unknown record type")]
    UnknownRecordType,
    #[error("signature error")]
    Signature(#[from] ethers::types::SignatureError),
}
