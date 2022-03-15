#!/bin/bash

cd lotus-compiler
cargo build
cd workshop
lotus-cli -r -d $@