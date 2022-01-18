#!/bin/bash

if [ "$1" == "--install" ] || [ "$1" == "-i" ] ; then
    cd lotus-cli && npm install
fi

if [ "$1" == "-d" ] ; then
    rm -rf game-test/build
fi

node --enable-source-maps --experimental-specifier-resolution=node lotus-cli/src/index.js --root game-test $@