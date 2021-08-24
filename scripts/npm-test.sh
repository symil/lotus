#!/bin/bash

./node_modules/.bin/mocha --experimental-wasi-unstable-preview1 --experimental-specifier-resolution=node scripts/run-all-tests.js $@