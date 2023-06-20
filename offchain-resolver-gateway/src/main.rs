#[cfg(feature = "diesel")]
use crate::db::diesel::DieselDatabase;
#[cfg(feature = "json")]
use crate::db::json::JsonDatabase;
use crate::db::Database;
use crate::gateway::Gateway;
use clap::{arg, command, value_parser, ArgGroup};
use color_eyre::Report;
use ethers::prelude::abigen;
use ethers::signers::{LocalWallet, Signer};
use eyre::Result;
use std::env;
use std::net::IpAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

mod db;
mod errors;
mod gateway;
mod utils;

abigen!(
    Resolver,
    "./res/Resolver.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

#[tokio::main]
async fn main() -> Result<(), Report> {
    let matches = command!()
        .about("ENS Offchain Gateway server answering requests from CCIP-READ protocol (aka ERC-3668)")
        .arg(
            arg!(-k --privatekey <VALUE> "private key of the wallet allowed to sign offchain ENS record results")
            .required(true)
            .env("PRIVATE_KEY")
            .hide_env_values(true)
        )
        .arg(arg!(-t --ttl <VALUE> "TTL for signatures")
            .value_parser(value_parser!(u64))
            .default_value("300")
            .env("TTL")
        )
        .arg(arg!(-i --ip <VALUE> "server IP to bind to -- change it to 0.0.0.0 for all interfaces")
            .value_parser(value_parser!(IpAddr))
            .default_value("127.0.0.1")
            .env("LISTEN_IP")
        )
        .arg(arg!(-p --port <VALUE> "server port to bind to")
            .value_parser(value_parser!(u16).range(1..))
            .default_value("8080")
            .env("LISTEN_PORT")
        )
        .arg(arg!(--json <FILE> "Json file to use as a database").value_parser(value_parser!(PathBuf)))
        .arg(arg!(--postgres <CONNECTION_STRING> "PostgreSQL connection string").env("DATABASE_URL"))
        .group(
            ArgGroup::new("database")
                .required(true)
                .args(["json", "postgres"]),
        )
        .get_matches();

    setup()?;

    let private_key = matches
        .get_one::<String>("privatekey")
        .expect("Missing private key");
    let ttl = *matches.get_one::<u64>("ttl").expect("Missing TTL");
    let ip_address = *matches.get_one::<IpAddr>("ip").expect("Missing IP address");
    let port = *matches.get_one::<u16>("port").expect("Missing port");

    let signer = private_key.parse::<LocalWallet>()?;
    info!("Signing wallet: {}", signer.address());

    let db = if matches.contains_id("json") {
        let file = matches.get_one::<PathBuf>("json").expect("Can't find file");
        info!("Using Json database from {:?}", file);
        let db = JsonDatabase::new(file);
        Arc::new(db) as Arc<dyn Database + Sync + Send>
    } else if matches.contains_id("postgres") {
        let database_url = matches
            .get_one::<String>("postgres")
            .expect("Database connection string missing");
        info!("Using Postgres database from {:?}", database_url);
        let db = DieselDatabase::new(database_url);
        Arc::new(db) as Arc<dyn Database + Sync + Send>
    } else {
        unreachable!();
    };
    let server = Gateway::new(signer, ttl, ip_address, port, db).await?;

    info!("Starting offchain resolver gateway...");
    server.start().await?;

    Ok(())
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .compact();
    let filter_layer = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
    Ok(())
}
