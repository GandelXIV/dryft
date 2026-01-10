#!/bin/bash

set -xe

RUSTFLAGS=-Awarnings cargo run -- example.dry -r

python3 test.py
