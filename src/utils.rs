use crate::errors::GatewayErrors;
use ethers::abi::{AbiEncode, Token};
use ethers::types::Signature;

/// Decode DNS name from call data, parsing ENS labels
pub fn decode_dns_name(dns_name: &Token) -> Result<String, GatewayErrors> {
    if let Some(dns_name) = dns_name.clone().into_bytes() {
        let mut labels = vec![];
        let mut idx = 0;
        loop {
            let len: usize = dns_name[idx].into();
            if len == 0 {
                break;
            }
            labels.push(std::str::from_utf8(&dns_name[idx + 1..(idx + len + 1)]).unwrap());
            idx += len + 1;
        }
        Ok(labels.join("."))
    } else {
        Err(GatewayErrors::InvalidName)
    }
}

/// Compute `yParityAndS` from a signature
///
/// EIP-2098: <https://eips.ethereum.org/EIPS/eip-2098>
pub fn compact_y_parity_and_s(sig: &Signature) -> Result<Vec<u8>, GatewayErrors> {
    let mut y_parity_and_s = sig.s.encode();
    if sig.recovery_id()?.is_y_odd() {
        y_parity_and_s[0] |= 0x80;
    }
    Ok(y_parity_and_s.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::{
        abi::AbiEncode,
        signers::{LocalWallet, Signer},
        types::Signature,
        utils::hex,
    };

    #[tokio::test]
    async fn test_y_parity_and_s() {
        let signer = "0x1234567890123456789012345678901234567890123456789012345678901234"
            .parse::<LocalWallet>()
            .unwrap();

        let sig: Signature = signer.sign_message("Hello World").await.unwrap();
        // check signature
        assert_eq!(
            "0x68a020a209d3d56c46f38cc50a33f704f4a9a10a59377f8dd762ac66910e9b90",
            sig.r.encode_hex()
        );
        assert_eq!(
            "0x7e865ad05c4035ab5792787d4a0297a43617ae897930a6fe4d822b8faea52064",
            sig.s.encode_hex()
        );
        assert_eq!(27, sig.v);
        // check yParityAndS
        let y_parity_and_s = compact_y_parity_and_s(&sig).unwrap();
        assert_eq!(
            "0x7e865ad05c4035ab5792787d4a0297a43617ae897930a6fe4d822b8faea52064",
            format!("0x{}", hex::encode(y_parity_and_s))
        );

        let sig: Signature = signer.sign_message("It's a small(er) world").await.unwrap();
        // check signature
        assert_eq!(
            "0x9328da16089fcba9bececa81663203989f2df5fe1faa6291a45381c81bd17f76",
            sig.r.encode_hex()
        );
        assert_eq!(
            "0x139c6d6b623b42da56557e5e734a43dc83345ddfadec52cbe24d0cc64f550793",
            sig.s.encode_hex()
        );
        assert_eq!(28, sig.v);
        // check yParityAndS
        let y_parity_and_s = compact_y_parity_and_s(&sig).unwrap();
        assert_eq!(
            "0x939c6d6b623b42da56557e5e734a43dc83345ddfadec52cbe24d0cc64f550793",
            format!("0x{}", hex::encode(y_parity_and_s))
        );
    }
}
