#!/bin/sh
git pull
cargo build --release --all
./build/release/tasks
service supervisor restart && service nginx restart
