#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
ROOT=$SCRIPT_DIR/..
INPUT_PATH=$SCRIPT_DIR/../client/js/index.js
SCRIPT_PATH=$SCRIPT_DIR/build.sh
FORMATTED_INPUT=`realpath --relative-to="$ROOT" "$INPUT_PATH"`
FORMATTED_SCRIPT=`realpath --relative-to="$ROOT" "$SCRIPT_PATH"`

unfold $FORMATTED_INPUT $FORMATTED_SCRIPT $@