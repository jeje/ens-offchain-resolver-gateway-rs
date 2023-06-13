use crate::types::CCIPReadHandler;
use crate::CCIPReadMiddlewareError;
use ethers_core::abi::{Abi, Function};
use serde::Deserialize;
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use tracing::debug;
use warp::Filter;

type Handlers = HashMap<[u8; 4], (Function, Arc<dyn CCIPReadHandler + Sync + Send>)>;

/// CCIP-Read Server.
#[derive(Clone)]
pub struct Server {
    ip_address: IpAddr,
    port: u16,
    handlers: Handlers,
}

#[derive(Deserialize)]
pub struct CCIPReadMiddlewareRequest {
    sender: String,
    calldata: String,
}

impl Server {
    /// Create a new server
    ///
    /// # Arguments
    /// * `port` the port the server should bind to
    pub fn new(ip_address: IpAddr, port: u16) -> Self {
        Server {
            ip_address,
            port,
            handlers: HashMap::new(),
        }
    }

    /// Add callbacks for CCIP-Read server requests
    ///
    /// # Arguments
    /// * `abi` the parsed ABI of the contract to decode data for
    /// * `handlers` the callbacks
    pub fn add(
        &mut self,
        abi: Abi,
        name: &str,
        callback: Arc<dyn CCIPReadHandler + Sync + Send>,
    ) -> Result<(), CCIPReadMiddlewareError> {
        let function = abi.function(name)?.clone();
        debug!(
            "Added function with short sig: {:?}",
            function.short_signature()
        );
        self.handlers
            .insert(function.short_signature(), (function, callback));
        Ok(())
    }

    /// Starts a new CCIP-Read server.
    pub async fn start(&self) -> Result<(), CCIPReadMiddlewareError> {
        let api = filters::gateway(self.handlers.clone());
        let routes = api.with(warp::log("gateway"));
        let bound_interface: SocketAddr = SocketAddr::new(self.ip_address, self.port);
        warp::serve(routes).run(bound_interface).await;
        Ok(())
    }
}

mod filters {
    use super::{handlers, CCIPReadMiddlewareRequest, Handlers};
    use warp::Filter;

    pub fn gateway(
        handlers: Handlers,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        get(handlers.clone()).or(post(handlers.clone()))
    }

    pub fn get(
        handlers: Handlers,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("gateway" / String / String)
            .and(warp::get())
            .and(with_handlers(handlers))
            .and_then(handlers::gateway_get)
    }

    pub fn post(
        handlers: Handlers,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("gateway")
            .and(warp::post())
            .and(json_body())
            .and(with_handlers(handlers))
            .and_then(handlers::gateway_post)
    }

    fn with_handlers(
        handlers: Handlers,
    ) -> impl Filter<Extract = (Handlers,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || handlers.clone())
    }

    fn json_body(
    ) -> impl Filter<Extract = (CCIPReadMiddlewareRequest,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}

mod handlers {
    use super::{CCIPReadMiddlewareRequest, Handlers};
    use crate::{
        types::{RPCCall, RPCResponse},
        CCIPReadMiddlewareError,
    };
    use ethers_core::utils::hex;
    use serde_json::json;
    use std::{convert::Infallible, str::FromStr};
    use tracing::debug;
    use warp::hyper::StatusCode;

    pub async fn gateway_get(
        sender: String,
        calldata: String,
        handlers: Handlers,
    ) -> Result<impl warp::Reply, Infallible> {
        let calldata = String::from(calldata.strip_suffix(".json").unwrap_or(calldata.as_str()));
        debug!("Should handle sender={:?} calldata={}", sender, calldata);

        if let Ok(calldata) = ethers_core::types::Bytes::from_str(&calldata.as_str()[2..]) {
            let response = call(
                RPCCall {
                    to: sender.clone(),
                    data: calldata,
                },
                handlers,
            )
            .await
            .unwrap();

            let body = response.body;
            Ok(warp::reply::with_status(
                warp::reply::json(&body),
                StatusCode::OK,
            ))
        } else {
            Ok(warp::reply::with_status(
                warp::reply::json(&json!({
                    "message": "Unexpected error",
                })),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }

    pub async fn gateway_post(
        data: CCIPReadMiddlewareRequest,
        handlers: Handlers,
    ) -> Result<impl warp::Reply, Infallible> {
        let sender = data.sender;
        let calldata = String::from(
            data.calldata
                .strip_suffix(".json")
                .unwrap_or(data.calldata.as_str()),
        );
        debug!("Should handle sender={:?} calldata={}", sender, calldata);

        if let Ok(calldata) = ethers_core::types::Bytes::from_str(&calldata.as_str()[2..]) {
            let response = call(
                RPCCall {
                    to: sender.clone(),
                    data: calldata,
                },
                handlers,
            )
            .await
            .unwrap();

            let body = response.body;
            Ok(warp::reply::with_status(
                warp::reply::json(&body),
                StatusCode::OK,
            ))
        } else {
            Ok(warp::reply::with_status(
                warp::reply::json(&json!({
                    "message": "Unexpected error",
                })),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }

    #[tracing::instrument(
        name = "ccip_server"
        skip_all
    )]
    async fn call(
        call: RPCCall,
        handlers: Handlers,
    ) -> Result<RPCResponse, CCIPReadMiddlewareError> {
        debug!("Received call with {:?}", call);
        let selector = &call.data[0..4];

        // find a function handler for this selector
        let handler = if let Some(handler) = handlers.get(selector) {
            handler
        } else {
            return Ok(RPCResponse {
                status: 404,
                body: json!({
                    "message": format!("No implementation for function with selector 0x{}", hex::encode(selector)),
                }),
            });
        };

        // decode function arguments
        let args = handler.0.decode_input(&call.data[4..])?;

        let callback = handler.1.clone();
        if let Ok(tokens) = callback
            .call(
                args,
                RPCCall {
                    to: call.to,
                    data: call.data,
                },
            )
            .await
        {
            let encoded_data = ethers_core::abi::encode(&tokens);
            let encoded_data = format!("0x{}", hex::encode(encoded_data));
            debug!("Final encoded data: {}", encoded_data);

            Ok(RPCResponse {
                status: 200,
                body: json!({
                    "data": encoded_data,
                }),
            })
        } else {
            Ok(RPCResponse {
                status: 500,
                body: json!({
                    "message": "Unexpected error",
                }),
            })
        }
    }
}

// Sample ENS offchain resolver request:
// http://localhost:8080/gateway/0x8464135c8f25da09e49bc8782676a84730c318bc/0x9061b92300000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000a047465737403657468000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000243b3b57deeb4f647bea6caa36333c816d7b46fdcb05f9466ecacc140ea8c66faf15b3d9f100000000000000000000000000000000000000000000000000000000.json
#[cfg(test)]
mod tests {
    use crate::server::Handlers;
    use ethers::abi::AbiParser;
    use ethers::contract::BaseContract;
    use serde_json::json;
    use std::collections::HashMap;
    use warp::hyper::body::Bytes;

    #[test]
    fn it_parse_offchain_resolver_abi() {
        let abi = AbiParser::default().parse_str(r#"[
            function resolve(bytes memory name, bytes memory data) external view returns(bytes memory)
        ]"#).unwrap();
        let contract = BaseContract::from(abi);
        println!("{:?}", contract.methods);
    }

    #[tokio::test]
    async fn test_gateway_get_on_unknown_selector() {
        let handlers: Handlers = HashMap::new();
        let filter = super::filters::get(handlers);

        let res = warp::test::request()
            .path("/gateway/0x8464135c8f25da09e49bc8782676a84730c318bc/0x9061b92300000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000a0474657374036574680000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008459d1d43ceb4f647bea6caa36333c816d7b46fdcb05f9466ecacc140ea8c66faf15b3d9f100000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000005656d61696c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.json")
            .reply(&filter)
            .await;
        assert_eq!(res.status(), 200);
        assert_eq!(
            res.body(),
            &Bytes::from(
                json!({ "message": "No implementation for function with selector 0x9061b923"})
                    .to_string()
            )
        );
    }
}
