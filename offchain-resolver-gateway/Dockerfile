FROM alpine:3.16.2 as builder

WORKDIR /opt/

RUN apk add curl protoc musl-dev gzip git

# offchain-resolver-gateway
RUN curl -sLO https://github.com/jeje/ens-offchain-resolver-gateway-rs/releases/latest/download/offchain-resolver-gateway-x86_64-unknown-linux-musl.tar.gz \
  && tar -xvf offchain-resolver-gateway-x86_64-unknown-linux-musl.tar.gz \
  && chmod +x offchain-resolver-gateway

#########################################################

FROM alpine:3.16.2

RUN apk add tmux

COPY --from=builder /opt/offchain-resolver-gateway /opt/offchain-resolver-gateway

RUN chown -R root:root /opt/

#ENV PRIVATE_KEY
ENV TTL 300
ENV LISTEN_IP 0.0.0.0
ENV LISTEN_PORT 8080

ENTRYPOINT [ "/opt/offchain-resolver-gateway" ]