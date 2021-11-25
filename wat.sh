#!/bin/bash

wat2wasm --enable-bulk-memory lotus-compiler/workshop/build/module.wat -o lotus-compiler/workshop/build/module.wasm 2>&1 > /dev/null | head -n 20