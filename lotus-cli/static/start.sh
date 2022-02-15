#!/bin/bash

PORT=8080
#PORT=X
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
DIR_NAME=$(basename -- $SCRIPT_DIR)
NOHUP_PID_DIR_PATH="$HOME/.nohup_pids"
NOHUP_PID_FILE_PATH="$NOHUP_PID_DIR_PATH/$DIR_NAME"

if [[ ! -z $1 ]] ; then
    PORT=$1
fi

if [[ -f $NOHUP_PID_FILE_PATH ]] ; then
    echo "stopping current process"
    kill -9 $(cat $NOHUP_PID_FILE_PATH)
fi

if [[ $1 == "--stop" ]] ; then
    rm -f $NOHUP_PID_FILE_PATH
else
    mkdir -p $HOME/.nohup_pids
    cd $SCRIPT_DIR
    echo "starting new process"
    nohup node entry-point.js $PORT > nohup.log 2>&1 &
    echo $! > $HOME/.nohup_pids/$DIR_NAME
fi