#!/bin/bash

cargo run -p lotus-parser
wat2wasm build/module.wat -o build/module.wasm

if [ "$1" == "-b" ]; then
    cat build/module.wat
    echo
else
    node --experimental-wasi-unstable-preview1 wasm.js
fi