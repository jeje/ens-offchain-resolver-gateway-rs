<!-- Badges -->
[![CI Status][ci-badge]][ci-url]
[![Docker][docker-badge]][docker-url]
<!-- [![Crates.io][crates-badge]][crates-url] -->
<!-- [![Docs.rs][docs-badge]][docs-url] -->

<!-- Badge Images -->
[ci-badge]: https://github.com/jeje/ens-offchain-resolver-gateway-rs/actions/workflows/ci.yml/badge.svg
[ci-url]: https://github.com/jeje/ens-offchain-resolver-gateway-rs/actions/workflows/ci.yml
[docker-badge]: https://img.shields.io/badge/docker-%230db7ed.svg?logo=docker&logoColor=white
[docker-url]: https://hub.docker.com/r/jeje/ens-offchain-resolver-gateway-rs
<!-- [crates-badge]: https://img.shields.io/crates/v/ethers-ccip-read.svg -->
<!--[crates-url]: https://crates.io/crates/ethers-ccip-read-->
<!--[docs-badge]: https://docs.rs/ethers-ccip-read/badge.svg-->
<!--[docs-url]: https://docs.rs/ethers-ccip-read-->


## ENS Offchain Resolver Gateway

This repository contains multiple things:
* ENS gateway implementation working in a similar way to the
  [TypeScript gateway implementation](https://github.com/ensdomains/offchain-resolver/tree/main/packages/gateway)
* [ERC-3668: CCIP Read](https://eips.ethereum.org/EIPS/eip-3668#gateway-interface)
  server **library** in order to develop some servers, like the provided ENS gateway \
  **Note:** deliver this library as a crate on `crates.io`

Precompiled ENS gateways are available in [releases page](https://github.com/jeje/ens-offchain-resolver-gateway-rs/releases).

A Docker image is also available: https://hub.docker.com/r/jeje/ens-offchain-resolver-gateway-rs

### CLI Usage

```
ENS Offchain Gateway server answering requests from CCIP-READ protocol (aka ERC-3668)

Usage: offchain-resolver-gateway [OPTIONS] --privatekey <VALUE> <--json <FILE>|--postgres <CONNECTION_STRING>>

Options:
  -k, --privatekey <VALUE>            private key of the wallet allowed to sign offchain ENS record results [env: PRIVATE_KEY]
  -t, --ttl <VALUE>                   TTL for signatures [env: TTL=] [default: 300]
  -i, --ip <VALUE>                    server IP to bind to -- change it to 0.0.0.0 for all interfaces [env: LISTEN_IP=] [default: 127.0.0.1]
  -p, --port <VALUE>                  server port to bind to [env: LISTEN_PORT=] [default: 8080]
      --json <FILE>                   Json file to use as a database
      --postgres <CONNECTION_STRING>  PostgreSQL connection string [env: DATABASE_URL=postgresql://postgres:mysecretpassword@localhost:5432/ens_domains]
  -h, --help                          Print help
  -V, --version                       Print version
```

### Docker Usage
```shell
PRIVATE_KEY="<your private key>" docker run --rm \
  -e PRIVATE_KEY=${PRIVATE_KEY} \
  -p 8080:8080 \
  -v .:/tmp:ro \
  jeje/ens-offchain-resolver-gateway-rs \
  --json /tmp/test.eth.json
```

### Storage Engines
Two storage engines are supported:
* static Json file (`JsonDatabase`)
* Postgresql, Mysql, Sqlite3 databases (`DieselDatabase`)

### Helpful Resources

* Ethers-rs CCIP-Read Middleware: https://github.com/ensdomains/ethers-ccip-read
* Contracts + Client + Gateways implementations (in TypeScript): \
  https://github.com/ensdomains/offchain-resolver
* EIP-3668 specification: https://eips.ethereum.org/EIPS/eip-3668