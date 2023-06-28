//! ENS offchain resolver gateway framework

/// Various offchain database storage options
pub mod db;
/// Errors
pub mod errors;
/// Gateway framework
pub mod gateway;
/// Utils
pub mod utils;

use ethers::prelude::abigen;

abigen!(
    Resolver,
    "./res/Resolver.json",
    event_derives(serde::Deserialize, serde::Serialize)
);
