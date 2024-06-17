#####
FROM clux/muslrust:stable AS builder

WORKDIR /app

COPY ./ .
RUN cargo build --release &&\
    cp ./target/x86_64-unknown-linux-musl/release/tcp-server-test  /usr/bin/

#####
FROM cgr.dev/chainguard/bash:latest
WORKDIR /app
COPY --from=builder /usr/bin/tcp-server-test .

VOLUME /data

ENTRYPOINT ["./tcp-server-test"]