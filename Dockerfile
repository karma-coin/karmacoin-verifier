FROM rust:1.70.0-slim-buster AS builder

RUN apt update \
    && apt install -y libssl-dev pkg-config protobuf-compiler openssl ca-certificates \
    && apt clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

COPY ./ ./karmacoin-verifier
RUN cd /karmacoin-verifier
WORKDIR /karmacoin-verifier

RUN cargo build --release -p server-app
RUN mkdir /out && cp target/release/server-app ../out/server-app

FROM debian:stable-20210902-slim AS runtime

RUN apt update \
    && apt install -y libssl-dev pkg-config protobuf-compiler openssl ca-certificates \
    && apt clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

COPY --from=builder /out/ /

EXPOSE 9080 9080

ENTRYPOINT ["/server-app"]