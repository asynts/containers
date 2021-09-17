#!/bin/bash
set -e

export RUST_BACKTRACE=1

RUSTFLAGS="-C target-feature=+crt-static" cargo build --bin asynts-jail-example --target x86_64-unknown-linux-musl
cargo build --bin asynts-jail
sudo RUST_BACKTRACE=1 ./target/debug/asynts-jail
