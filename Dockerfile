# Use the official Rust base image
FROM rust:1.82-slim-bookworm AS builder

# Install build dependencies
RUN echo "deb http://deb.debian.org/debian testing main" | tee /etc/apt/sources.list.d/testing.list
RUN apt-get update -yq
RUN apt-get install -y ca-certificates pkg-config libssl-dev gcc-13 g++-13 libopenblas-dev
RUN update-alternatives --install /usr/bin/gcc gcc /usr/bin/gcc-13 60 \
  --slave /usr/bin/g++ g++ /usr/bin/g++-13 \
  --slave /usr/bin/gcov gcov /usr/bin/gcov-13

ENV CC=/usr/bin/gcc-13
ENV CXX=/usr/bin/g++-13

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
RUN echo "deb http://deb.debian.org/debian testing main" | tee /etc/apt/sources.list.d/testing.list
RUN apt-get update && \
  apt-get install -y ca-certificates libssl-dev gcc-13 libopenblas-dev && \
  rm -rf /var/lib/apt/lists/* && \
  update-ca-certificates

RUN useradd -m -s /bin/bash pacmap
USER pacmap

WORKDIR /home/pacmap
COPY --from=builder /usr/src/pacmap-example/target/release/pacmap-rs-example /usr/bin/pacmap-rs-example

CMD ["/usr/bin/pacmap-rs-example"]
VOLUME ["/home/pacmap"]