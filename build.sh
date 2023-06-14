#!/usr/bin/env bash

set -xe

cargo build --package qrazy_wasm --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/qrazy_wasm.wasm example-site/qrazy_wasm.wasm
