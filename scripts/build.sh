#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

killall lotus-server 2> /dev/null
cd $SCRIPT_DIR/../client && wasm-pack build --target bundler && cd $SCRIPT_DIR/../server && cargo run