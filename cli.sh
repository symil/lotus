#!/bin/bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
TEST_GAME_PATH=$SCRIPT_DIR/game-test

if [ "$1" == "-d" ] ; then
    rm -rf $TEST_GAME_PATH/build
    exit
fi

cd $TEST_GAME_PATH
lotus-cli $@