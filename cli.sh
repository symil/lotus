#!/bin/bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
TEST_GAME_PATH=$SCRIPT_DIR/game-test

if [ "$1" == "-d" ] ; then
    rm -rf $TEST_GAME_PATH/build
    exit
fi

cd $TEST_GAME_PATH
node --enable-source-maps --experimental-specifier-resolution=node $SCRIPT_DIR/lotus-compiler/javascript/cli.js 33902 az@ytawo.eu:221 $@