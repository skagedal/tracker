#!/usr/bin/env bash

# This isn't much of a general install script, it just hardcodes what I personally want installed.

set -e

cargo install --path .

if [ -d ~/.oh-my-zsh ]; then
    ln -fs `pwd`/shell/tracker-function.zsh ~/.oh-my-zsh/custom/tracker-function.zsh
fi

if [ -d ~/local/zsh-functions ]; then
    cargo run -- completions zsh > ~/local/zsh-functions/_tracker
fi
