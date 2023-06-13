//use ethers_providers::{Middleware, MiddlewareError};
use thiserror::Error;

/// Handle CCIP-Read middleware specific errors.
#[derive(Error, Debug)]
pub enum CCIPReadMiddlewareError /*<M: Middleware>*/ {
    #[error("Unknown function")]
    UnknownFunction(#[from] ethers_core::abi::Error),
    /*
    /// Thrown when the internal middleware errors
    #[error("{0}")]
    MiddlewareError(M::Error),
    */
    #[error("Parsing error")]
    Parsing(#[from] serde_json::Error),

    #[error("Abi error")]
    Abi(#[from] ethers_core::abi::AbiError),

    #[error("Parse bytes error")]
    ParseBytes(#[from] ethers_core::types::ParseBytesError),

    #[error("URL parse error")]
    UrlParse(#[from] url::ParseError),
}

/*
impl<M: Middleware> MiddlewareError for CCIPReadMiddlewareError<M> {
    type Inner = M::Error;

    fn from_err(src: M::Error) -> Self {
        CCIPReadMiddlewareError::MiddlewareError(src)
    }

    fn as_inner(&self) -> Option<&Self::Inner> {
        match self {
            CCIPReadMiddlewareError::MiddlewareError(e) => Some(e),
            _ => None,
        }
    }
}
*/
