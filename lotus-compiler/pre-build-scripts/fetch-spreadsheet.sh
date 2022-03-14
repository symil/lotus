#!/bin/bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

export LOTUS_SPREADSHEET_URL="$1"
export LOTUS_CACHE_DIR="$2"

node --experimental-specifier-resolution=node $SCRIPT_DIR/../javascript/fetch-spreadsheet.js