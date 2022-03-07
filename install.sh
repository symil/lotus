#!/bin/bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

sudo ln -sf $SCRIPT_DIR/scripts/run-cli.js /usr/local/bin/lotus-cli
sudo chmod +x /usr/local/bin/lotus-cli