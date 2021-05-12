#!/usr/bin/env bash

set -e

TOOL=tracker
BIN=${HOME}/local/bin
BUILT_BINARY=`pwd`/${TOOL}/build/install/${TOOL}/bin/${TOOL}

./gradlew install
ln -fs ${BUILT_BINARY} ${BIN}/${TOOL}

if [ -d ~/.oh-my-zsh ]; then
    _TRACKER_COMPLETE=zsh ${BUILT_BINARY} > ~/.oh-my-zsh/custom/tracker.zsh
    ln -fs `pwd`/shell/tracker-function.zsh ~/.oh-my-zsh/custom/tracker-function.zsh
fi

