use async_trait::async_trait;
use ethers::types::{Address, U256};
use serde_json::Value;
use std::{fs::File, io::BufReader, path::PathBuf};
use tracing::info;

use super::{AddressBytesRecord, AddressRecord, ContentHashRecord, Database, TextRecord};

pub struct JsonDatabase {
    json: Value,
}

impl JsonDatabase {
    pub fn new(file: &PathBuf) -> Self {
        let file = File::open(file).expect("Can't open file");
        let reader = BufReader::new(file);
        let json = serde_json::from_reader(reader).expect("Can't parse Json file");
        JsonDatabase { json }
    }

    fn get_domain(&self, name: &str) -> &Value {
        &self.json[name]
    }
}

#[async_trait]
impl Database for JsonDatabase {
    #[tracing::instrument(
        name = "db::addr"
        skip(self)
    )]
    async fn addr(&self, name: &str) -> Option<AddressRecord> {
        info!(tag = "db", "Searching addr record");
        let domain = self.get_domain(name);
        domain["addresses"]["60"]
            .as_str()
            .map(|value| AddressRecord {
                addr: value.parse().map_err(|_| Address::zero()).unwrap(),
            })
    }

    #[tracing::instrument(
        name = "db::add_coin_type"
        skip(self)
    )]
    async fn addr_coin_type(&self, name: &str, coin_type: U256) -> Option<AddressBytesRecord> {
        info!(tag = "db", "Searching multicoin addr record");
        let domain = self.get_domain(name);
        domain["addresses"][coin_type.to_string()]
            .as_str()
            .map(|value| AddressBytesRecord {
                addr: value.parse().map_err(|_| Address::zero()).unwrap(),
            })
    }

    #[tracing::instrument(
        name = "db::text"
        skip(self)
    )]
    async fn text(&self, name: &str, key: &str) -> Option<TextRecord> {
        info!(tag = "db", "Searching text record");
        let domain = self.get_domain(name);
        domain["text"][key].as_str().map(|value| TextRecord {
            value: String::from(value),
        })
    }

    #[tracing::instrument(
        name = "db::content_hash"
        skip(self)
    )]
    async fn contenthash(&self, name: &str) -> Option<ContentHashRecord> {
        info!(tag = "db", "Searching content hash record");
        let domain = self.get_domain(name);
        domain["contenthash"]
            .as_str()
            .map(|value| ContentHashRecord {
                content_hash: String::from(if value.starts_with("0x") {
                    value.get(2..).unwrap()
                } else {
                    value
                }),
            })
    }
}
