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

ENS gateway implementation working in a similar way to the
[TypeScript gateway implementation](https://github.com/ensdomains/offchain-resolver/tree/main/packages/gateway).

Precompiled ENS gateways are available in [releases page](https://github.com/jeje/ens-offchain-resolver-gateway-rs/releases).

A Docker image is also available: https://hub.docker.com/r/jeje/ens-offchain-resolver-gateway-rs

Lastly a library is provided to ease implementation of custom gateways without duplicating much code.
A good sample is the [default implementation provided](src/main.rs).

### CLI Usage

```
ENS Offchain Gateway server answering requests from CCIP-READ protocol (aka ERC-3668)

Usage: ens-gateway [OPTIONS] --privatekey <VALUE> <--json <FILE>>

Options:
  -k, --privatekey <VALUE>  private key of the wallet allowed to sign offchain ENS record results [env: PRIVATE_KEY]
  -t, --ttl <VALUE>         TTL for signatures [env: TTL=] [default: 300]
  -i, --ip <VALUE>          server IP to bind to -- change it to 0.0.0.0 for all interfaces [env: LISTEN_IP=] [default: 127.0.0.1]
  -p, --port <VALUE>        server port to bind to [env: LISTEN_PORT=] [default: 8080]
      --json <FILE>         Json file to use as a database
  -h, --help                Print help
  -V, --version             Print version
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

### Helpful Resources

* Ethers-rs CCIP-Read Middleware: https://github.com/ensdomains/ethers-ccip-read
* Contracts + Client + Gateways implementations (in TypeScript): \
  https://github.com/ensdomains/offchain-resolver
* EIP-3668 specification: https://eips.ethereum.org/EIPS/eip-3668