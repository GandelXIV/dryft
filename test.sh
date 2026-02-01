#!/bin/bash

set -xe

TS="python3 test/test.py"
CR="cargo run --"

RUSTFLAGS=-Awarnings cargo test
RUSTFLAGS=-Awarnings cargo test --features typesystem

$CR example.dry
$TS test/example.txt

$CR examples/fizzbuzz.dry
$TS test/fizzbuzz.txt
