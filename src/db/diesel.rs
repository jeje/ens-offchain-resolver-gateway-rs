use super::diesel_schema::ens_records::dsl::*;
use super::{AddressBytesRecord, AddressRecord, ContentHashRecord, Database, TextRecord};
use async_trait::async_trait;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use ethers::types::{Address, U256};
use tracing::{debug, info};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub struct DieselDatabase {
    pool: Pool<ConnectionManager<PgConnection>>,
}

#[derive(diesel_derive_enum::DbEnum, Debug, PartialEq)]
#[ExistingTypePath = "crate::db::diesel_schema::sql_types::RecordType"]
enum RecordType {
    Address,
    Text,
    ContentHash,
}

#[derive(Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = crate::db::diesel_schema::ens_records)]
struct EnsRecord {
    id: i32,
    domain: String,
    record_type: RecordType,
    address_record_coin_type: Option<i64>,
    address_record_value: Option<String>,
    text_record_key: Option<String>,
    text_record_value: Option<String>,
    content_hash_record: Option<String>,
}

impl DieselDatabase {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url.to_owned());
        let pool = Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .expect("Could not build connection pool");
        // run migrations
        let mut connection = pool.get().unwrap();
        connection.run_pending_migrations(MIGRATIONS).unwrap();
        DieselDatabase { pool }
    }
}

#[async_trait]
impl Database for DieselDatabase {
    #[tracing::instrument(
        name = "db::addr"
        skip(self)
    )]
    async fn addr(&self, name: &str) -> Option<AddressRecord> {
        info!(tag = "db", "Searching addr record");
        let connection = &mut self.pool.get().unwrap();
        let result: Option<EnsRecord> = ens_records
            .filter(domain.eq(name))
            .filter(record_type.eq(RecordType::Address))
            .filter(address_record_coin_type.eq(60))
            .limit(1)
            .select(EnsRecord::as_select())
            .first(connection)
            .ok();
        debug!(tag = "db", "Record: {:?}", result);
        if let Some(record) = result {
            record.address_record_value.map(|value| AddressRecord {
                addr: value.parse().map_err(|_| Address::zero()).unwrap(),
            })
        } else {
            None
        }
    }

    #[tracing::instrument(
        name = "db::add_coin_type"
        skip(self)
    )]
    async fn addr_coin_type(&self, name: &str, coin_type: U256) -> Option<AddressBytesRecord> {
        info!(tag = "db", "Searching multicoin addr record");
        let connection = &mut self.pool.get().unwrap();
        let result: Option<EnsRecord> = ens_records
            .filter(domain.eq(name))
            .filter(record_type.eq(RecordType::Address))
            .filter(address_record_coin_type.eq(coin_type.as_u64() as i64))
            .limit(1)
            .select(EnsRecord::as_select())
            .first(connection)
            .ok();
        debug!(tag = "db", "Record: {:?}", result);
        if let Some(record) = result {
            record.address_record_value.map(|value| AddressBytesRecord {
                addr: value.parse().map_err(|_| Address::zero()).unwrap(),
            })
        } else {
            None
        }
    }

    #[tracing::instrument(
        name = "db::text"
        skip(self)
    )]
    async fn text(&self, name: &str, key: &str) -> Option<TextRecord> {
        info!(tag = "db", "Searching text record");
        let connection = &mut self.pool.get().unwrap();
        let result: Option<EnsRecord> = ens_records
            .filter(domain.eq(name))
            .filter(record_type.eq(RecordType::Text))
            .filter(text_record_key.eq(key))
            .limit(1)
            .select(EnsRecord::as_select())
            .first(connection)
            .ok();
        debug!(tag = "db", "Record: {:?}", result);
        if let Some(record) = result {
            record.text_record_value.map(|value| TextRecord { value })
        } else {
            None
        }
    }

    #[tracing::instrument(
        name = "db::content_hash"
        skip(self)
    )]
    async fn contenthash(&self, name: &str) -> Option<ContentHashRecord> {
        info!(tag = "db", "Searching content hash record");
        let connection = &mut self.pool.get().unwrap();
        let result: Option<EnsRecord> = ens_records
            .filter(domain.eq(name))
            .filter(record_type.eq(RecordType::ContentHash))
            .limit(1)
            .select(EnsRecord::as_select())
            .first(connection)
            .ok();
        debug!(tag = "db", "Record: {:?}", result);
        if let Some(record) = result {
            record.content_hash_record.map(|value| ContentHashRecord {
                content_hash: String::from(if value.starts_with("0x") {
                    value.get(2..).unwrap()
                } else {
                    &value
                }),
            })
        } else {
            None
        }
    }
}
