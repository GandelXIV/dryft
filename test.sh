#!/bin/bash

set -xe

TS="python3 test/test.py"

RUSTFLAGS=-Awarnings cargo test

RUSTFLAGS=-Awarnings cargo run -- example.dry -r
$TS test/example.txt

RUSTFLAGS=-Awarnings cargo run -- examples/fizzbuzz.dry -r
$TS test/fizzbuzz.txt
