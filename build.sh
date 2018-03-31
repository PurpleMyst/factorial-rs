#!/usr/bin/env sh

set -e
cargo build --release
cp target/release/libfactorial_rs.so ./factorial.so
