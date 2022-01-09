#!/bin/bash

cd lotus-compiler

if [ "$1" == "-s" ] ; then
    cargo run -- --server
else
    cargo build
fi