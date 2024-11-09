# Use the official Rust base image
FROM rust:1.82-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update
RUN apt-get install -y ca-certificates pkg-config libssl-dev libopenblas-dev

# Create a new directory for the application
WORKDIR /usr/src/pacmap-example

# Fetch and cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY .cargo ./.cargo
COPY src ./src
RUN cargo fetch

# Build the example
RUN cargo build --release

FROM debian:bookworm-slim

# Set up the runtime environment, including required certificates and libraries
RUN apt-get update && \
  apt-get install -y ca-certificates libssl-dev libopenblas-dev && \
  rm -rf /var/lib/apt/lists/* && \
  update-ca-certificates

RUN useradd -m -s /bin/bash pacmap
USER pacmap

WORKDIR /home/pacmap
COPY --from=builder /usr/src/pacmap-example/target/release/pacmap-rs-example /usr/bin/pacmap-rs-example

CMD ["/usr/bin/pacmap-rs-example"]
VOLUME ["/home/pacmap"]
