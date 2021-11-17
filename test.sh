#!/bin/bash

set -e

if [ "$1" == "--backtrace" ] || [ "$1" == "-b" ] ; then
    export RUST_BACKTRACE=1
fi

node --experimental-specifier-resolution=node lotus-compiler/run-test.js $@