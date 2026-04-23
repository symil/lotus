#!/usr/bin/bash

set -a

cd $(dirname "$0")
source build.env

bun server-bundle.js