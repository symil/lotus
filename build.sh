#!/bin/bash

cd lotus-compiler

if [ "$1" == "-s" ] ; then
    cargo run -- --server
elif [ "$1" == "-r" ] ; then
    cargo build --release
else
    cargo build
fi