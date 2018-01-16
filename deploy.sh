#!/bin/sh
git pull
cargo build --release --all
./build/release/fetch
service supervisor restart && service nginx restart
