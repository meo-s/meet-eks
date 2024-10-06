FROM clux/muslrust:stable as build

RUN set -eux; \
    groupadd -r -g 50051 meos; \
    useradd -r -g meos -u 50051 meos

COPY --chown=meos:meos Cargo.toml /workspace/Cargo.toml
COPY --chown=meos:meos proto /workspace/proto
COPY --chown=meos:meos foo-service /workspace/foo-service
COPY --chown=meos:meos bar-service /workspace/bar-service
WORKDIR /workspace/bar-service

RUN USER=bar set -eux; \
    cargo build --release --target x86_64-unknown-linux-musl --bin bar-service


FROM alpine:3.20

RUN set -eux; \
    addgroup -g 50051 meos; \
    adduser -D -G meos -u 50051 meos

COPY --chown=meos:meos --from=build \
    /workspace/target/x86_64-unknown-linux-musl/release/bar-service \
    /opt/bar-service/bin/bar-service

EXPOSE 50051
CMD ["/opt/bar-service/bin/bar-service"]
