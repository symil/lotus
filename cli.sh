#!/bin/bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
TEST_GAME_PATH=$SCRIPT_DIR/../scaelya

if [ "$1" == "-d" ] ; then
    rm -rf $TEST_GAME_PATH/build
    exit
fi

if [ "$1" == "-b" ] ; then
    cd lotus-compiler
    cargo build
    cd $TEST_GAME_PATH
    export RUST_BACKTRACE=1
    lotus-cli -b -d
else
    cd $TEST_GAME_PATH
    lotus-cli $@
fi