#!/bin/bash

set -e

node --experimental-wasi-unstable-preview1 --experimental-specifier-resolution=node scripts/generate-test.js $@