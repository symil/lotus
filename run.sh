#!/bin/bash

set -e
cargo run -p lotus-parser
wat2wasm build/module.wat -o build/module.wasm

if [ "$1" != "-b" ]; then
    node --experimental-wasi-unstable-preview1 wasm.js
fi