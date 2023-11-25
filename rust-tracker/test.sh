#!/usr/bin/env bash
cargo build
cargo test
TESTWEEKFILE=test.txt
rm -f $TESTWEEKFILE
./target/debug/tracker -w $TESTWEEKFILE start
