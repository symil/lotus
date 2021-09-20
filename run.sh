#!/bin/bash

set -e

if [ "$1" == "--backtrace" ] || [ "$1" == "-b" ] ; then
    export RUST_BACKTRACE=1
fi

node --experimental-wasi-unstable-preview1 --experimental-specifier-resolution=node scripts/generate-test.js $@