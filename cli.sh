#!/bin/bash

if [ "$1" == "--install" ] || [ "$1" == "-i" ] ; then
    cd lotus-cli && npm install
fi

node --enable-source-maps --experimental-specifier-resolution=node lotus-cli/src/index.js --root game-test $@