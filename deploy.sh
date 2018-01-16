#!/bin/sh
git pull
cargo build --release
service supervisor restart && service nginx restart
