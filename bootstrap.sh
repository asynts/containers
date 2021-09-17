#!/bin/bash
set -e

RUSTFLAGS="-C target-feature=+crt-static" cargo build --bin asynts-jail-example --target x86_64-unknown-linux-musl
cargo build --bin asynts-jail
sudo ./target/debug/asynts-jail
