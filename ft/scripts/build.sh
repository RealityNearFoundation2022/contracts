#!/bin/bash
set -e
cd ../ft
# cd "`dirname $0`"
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
# cargo build --all --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/*.wasm ../res/
