#!/usr/bin/bash

set -a

cd $(dirname "$0")
source .env

bun server-bundle.js