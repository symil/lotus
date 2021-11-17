#!/bin/bash

if [ "$1" == "--install" ] || [ "$1" == "-i" ] ; then
    cd lotus-cli && npm install
fi

node --experimental-specifier-resolution=node lotus-cli/src/index.js $@