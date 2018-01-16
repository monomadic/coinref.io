#!/bin/sh
git pull
cargo build --release --all
./target/release/tasks
service supervisor restart && service nginx restart
