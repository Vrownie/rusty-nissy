#!/bin/bash

# Navigate to the rusty directory
cd rusty

# Build the Rust project in release mode
cargo build --release

# Copy the built library to the src directory
cp target/release/librusty.a ../src

# Generate C bindings
cbindgen --config cbindgen.toml --output ../src/rusty_utils.h

# Navigate back to the original directory
cd -

# Run make to build the project
make