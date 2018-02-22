#!/bin/sh
# cargo build --target=x86_64-unknown-linux-musl --release
cargo build --all
cargo run --bin coinref-import
