#!/usr/bin/env bash
cargo build
TESTWEEKFILE=test.txt
rm -f $TESTWEEKFILE

echo ">> Starting tracking..."
./target/debug/tracker -w $TESTWEEKFILE start
echo ">> Started tracking, now file is:"
cat $TESTWEEKFILE

echo ">> Stopping tracking..."
./target/debug/tracker -w $TESTWEEKFILE stop
echo ">> Stopped tracking, now file is:"
cat $TESTWEEKFILE