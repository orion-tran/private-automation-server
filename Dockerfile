FROM rust:alpine AS builder

RUN apk add --no-cache mold musl-dev

WORKDIR /usr/build
COPY src src
COPY Cargo.toml .
COPY Cargo.lock .

RUN ARCH=$(uname -m) && \
    RUSTFLAGS="-C link-arg=-fuse-ld=mold" \
    cargo build --target="$ARCH-unknown-linux-musl" --release

RUN ARCH=$(uname -m) && \
    BINARY="target/$ARCH-unknown-linux-musl/release/private-automation-server" && \
    readelf -p .comment $BINARY | grep -q "mold" && \
    $BINARY --version | grep -q "private-automation-server" && \
    ldd $BINARY 2>&1 | grep -q "Not a valid dynamic program" && \
    cp $BINARY /usr/bin

FROM alpine:latest

RUN apk add --no-cache ca-certificates tzdata && \
    update-ca-certificates && \
    adduser -DH -g "" -u 1000 runner

COPY --from=builder /usr/bin/private-automation-server /usr/bin/private-automation-server

USER runner
CMD ["private-automation-server"]
