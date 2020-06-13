# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build
RUN apt-get update
RUN apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /usr/src/daemon
COPY Cargo.toml Cargo.toml
RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
RUN rm -f target/x86_64-unknown-linux-musl/release/deps/daemon*
COPY src src
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest
# RUN addgroup -g 1000 idaemon
# RUN adduser -D -s /bin/sh -u 1000 -G idaemon idaemon
# WORKDIR /home/idaemon/bin/
COPY --from=cargo-build /usr/src/daemon/target/x86_64-unknown-linux-musl/release/daemon .

# RUN chown idaemon:idaemon daemon

# USER idaemon
CMD ["./daemon"]