#!/bin/bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

set -e

cd $SCRIPT_DIR/lotus-compiler
npm install
rustup default nightly
cargo build --release
sudo ln -sf $SCRIPT_DIR/scripts/run-cli.js /usr/local/bin/lotus-cli
sudo chmod +x /usr/local/bin/lotus-cli
cd - > /dev/null
echo -e "\e[1m\e[34mDone!\e[0m"