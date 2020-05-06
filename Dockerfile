FROM rust

COPY src src
COPY Cargo.toml Cargo.toml

RUN cargo build --release
RUN mkdir -p /app
RUN cp ./target/release/daemon /app

CMD /app/daemon