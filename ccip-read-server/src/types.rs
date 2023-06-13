use async_trait::async_trait;
use ethers_core::{abi::Token, types::Bytes};
use serde::Serialize;
use serde_json::Value;
use std::sync::Arc;

/// RPC Call data.
#[derive(Debug, Clone)]
pub struct RPCCall {
    pub to: String,
    pub data: Bytes,
}

/// RPC Call response with status.
#[derive(Serialize)]
pub struct RPCResponse {
    pub status: u32,
    pub body: Value,
}

#[async_trait]
pub trait CCIPReadHandler {
    /// Closure called by the server.
    ///
    /// # Arguments
    /// * `args` the parsed ABI input parameters
    /// * `req` the RPC call data
    async fn call(
        &self,
        args: Vec<Token>,
        req: RPCCall,
    ) -> Result<Vec<Token>, Box<dyn std::error::Error>>;
}

/// Callback from CCIP-Read server.
#[derive(Debug)]
pub struct HandlerCallback<'a, T: CCIPReadHandler> {
    /// Name of the smart-contract function
    pub name: &'a str,
    /// Closure called by the server
    pub function: Arc<T>,
}

#[derive(Clone, Debug)]
pub struct HandlerDescription<T: CCIPReadHandler> {
    pub name: &'static str,
    pub function: ethers_core::abi::Function,
    pub callback: Arc<T>,
}
