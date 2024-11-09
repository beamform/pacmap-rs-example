# PaCMAP Rust Example

This repository contains an example of using the Rust implementation of PaCMAP (Pairwise Controlled Manifold
Approximation), a dimensionality reduction and visualization algorithm. This example demonstrates how to reduce the USPS
digits dataset from high-dimensional space to 2D and create an interactive visualization.

## Prerequisites

You'll need:

- A Unix-like operating system (Linux, macOS, WSL on Windows)
- An internet connection (for downloading Rust and the dataset)
- Basic familiarity with command line operations

## Getting Started

1. Clone this repository:

```bash
git clone https://github.com/beamform/pacmap-rs-example
cd pacmap-rs-example
```

2. Install Rust using the provided Makefile:

```bash
make install
```

After installation, you'll need to either restart your terminal or run:

```bash
source $HOME/.cargo/env
```

## Building and Running

The project includes a Makefile with several helpful commands:

- Build the project in development mode:

```bash
make build
```

- Build the project in release mode (recommended for better performance):

```bash
make build-release
```

- Run the example (will build in release mode):

```bash
make run
```

## What the Example Does

When you run the example:

1. It downloads the USPS digits dataset
2. Applies PaCMAP dimensionality reduction
3. Creates an interactive visualization saved as `pacmap_visualization.html`

The visualization will show the USPS digits dataset reduced to 2D, with points colored by their digit class.

## Project Structure

- `src/main.rs` - The main example code
- `Cargo.toml` - Project dependencies and configuration
- `Makefile` - Build and run commands

## Cleaning Up

To remove build artifacts:

```bash
make clean
```

## Troubleshooting

If you encounter any issues:

1. Make sure you have an internet connection for downloading the dataset
2. Verify Rust is installed correctly with `rustc --version`
3. Try cleaning and rebuilding: `make clean && make build-release`

## Additional Resources

- [Rust PaCMAP Documentation](https://docs.rs/pacmap)
- [Original PaCMAP Library](https://github.com/YingfanWang/PaCMAP)
- [Original PaCMAP Paper](https://jmlr.org/papers/v22/20-1061.html)
- [Rust Programming Language](https://www.rust-lang.org/)
- [Rust Book](https://rust-book.cs.brown.edu/title-page.html)
- [Cargo Book](https://doc.rust-lang.org/cargo/)