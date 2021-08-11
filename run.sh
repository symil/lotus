#!/bin/bash

set -e

cargo run -p lotus-parser -- test build/module.wat
wat2wasm build/module.wat -o build/module.wasm

if [ "$1" != "-b" ]; then
    node --experimental-wasi-unstable-preview1 scripts/run-wasm.js build/module.wasm
fi