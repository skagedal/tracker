#!/usr/bin/env bash
cargo build
TESTWEEKFILE=test.txt
rm -f $TESTWEEKFILE

echo ">> Starting tracking..."
./target/debug/tracker -w $TESTWEEKFILE start
echo ">> Started tracking, now file is:"
cat $TESTWEEKFILE

TRACKER=./target/debug/tracker

echo ">> Stopping tracking..."
$TRACKER -w $TESTWEEKFILE stop
echo ">> Stopped tracking, now file is:"
cat $TESTWEEKFILE

echo ">> Let's do it all again..."

echo ">> Starting tracking..."
$TRACKER -w $TESTWEEKFILE start
echo ">> Started tracking, now file is:"
cat $TESTWEEKFILE

echo ">> Stopping tracking..."
$TRACKER -w $TESTWEEKFILE stop
echo ">> Stopped tracking, now file is:"
cat $TESTWEEKFILE

echo ">> Editing..."
$TRACKER -w $TESTWEEKFILE edit
