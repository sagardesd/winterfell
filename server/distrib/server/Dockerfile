FROM rust:1.57-slim-buster AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        build-essential \
        curl \
        libssl-dev \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/app/src


# Copy your source tree
COPY ../../src ./src
COPY ../../data ./
COPY ../../Cargo.lock ../
COPY ../../Cargo.toml ../

# Build for release
RUN rm ./target/release/deps/main
RUN cargo build --release

# Set the startup command to run your binary
CMD ["/usr/src/target/release/main"]
