#!/bin/bash

set -xe

TS="python3 test/test.py"
CR="cargo run --"

RUSTFLAGS=-Awarnings cargo test

$CR example.dry
$TS test/example.txt

$CR examples/fizzbuzz.dry
$TS test/fizzbuzz.txt
