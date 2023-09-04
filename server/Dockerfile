FROM rust:latest
WORKDIR /winterfell

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN mkdir src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src/

COPY ./src ./src
COPY ./data ./data

RUN rm -rf ./target/release/deps/main*
RUN cargo build --release

CMD ["./target/release/main"]

