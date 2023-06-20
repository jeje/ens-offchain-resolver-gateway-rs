use async_trait::async_trait;
use ethers::types::{Address, Bytes, U256};

#[cfg(feature = "diesel")]
pub mod diesel;
#[cfg(feature = "diesel")]
pub mod diesel_schema;
#[cfg(feature = "json")]
pub mod json;

#[derive(Debug)]
pub struct AddressRecord {
    pub addr: Address,
}

pub struct AddressBytesRecord {
    pub addr: Bytes,
}

#[derive(Debug)]
pub struct TextRecord {
    pub value: String,
}

#[derive(Debug)]
pub struct ContentHashRecord {
    pub content_hash: String,
}

#[async_trait]
pub trait Database {
    async fn addr(&self, name: &str) -> Option<AddressRecord>;
    async fn addr_coin_type(&self, name: &str, coin_type: U256) -> Option<AddressBytesRecord>;
    async fn text(&self, name: &str, key: &str) -> Option<TextRecord>;
    async fn contenthash(&self, name: &str) -> Option<ContentHashRecord>;
}
