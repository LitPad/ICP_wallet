#!/bin/sh

echo Building package $1
cargo build --target wasm32-unknown-unknown --release --package $1