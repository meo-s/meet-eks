FROM clux/muslrust:stable as build

RUN set -eux; \
    groupadd -r -g 50051 meos; \
    useradd -r -g meos -u 50051 meos

COPY --chown=meos:meos Cargo.toml /workspace/Cargo.toml
COPY --chown=meos:meos proto /workspace/proto
COPY --chown=meos:meos foo-service /workspace/foo-service
COPY --chown=meos:meos bar-service /workspace/bar-service
WORKDIR /workspace/foo-service

RUN USER=foo set -eux; \
    cargo build --release --target x86_64-unknown-linux-musl --bin foo-service


FROM alpine:3.20

RUN set -eux; \
    addgroup -g 50051 meos; \
    adduser -D -G meos -u 50051 meos

COPY --chown=meos:meos --from=build \
    /workspace/target/x86_64-unknown-linux-musl/release/foo-service \
    /opt/foo-service/bin/foo-service

EXPOSE 50051
CMD ["/opt/foo-service/bin/foo-service"]
