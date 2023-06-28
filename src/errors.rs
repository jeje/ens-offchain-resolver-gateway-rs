use thiserror::Error;

/// ENS gateway errors
#[derive(Error, Debug)]
pub enum GatewayErrors {
    /// Invalid ENS name error
    #[error("invalid ENS name")]
    InvalidName,
    /// ABI error
    #[error("ABI error {0}")]
    Abi(#[from] ethers::contract::AbiError),
    /// ABI Parsing error
    #[error("ABI parse error {0}")]
    AbiParse(#[from] ethers::abi::ParseError),
    /// CCIP-Read server error
    #[error("CCIP Read error {0}")]
    CCIPReadMiddleware(#[from] ccip_read_server::CCIPReadMiddlewareError),
    /// Unknown ENS record type error
    #[error("unknown record type")]
    UnknownRecordType,
    /// Invalid signature error
    #[error("signature error")]
    Signature(#[from] ethers::types::SignatureError),
}
