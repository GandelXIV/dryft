#!/usr/bin/python3
import subprocess
import difflib
import sys

result = subprocess.run(
    ["./a.out"],
    capture_output=True,
    text=True
)

def rf(name):
    with open(name, 'r') as f:
        return f.read()

expected = rf(sys.argv[1])

diff = difflib.unified_diff(
    expected.splitlines(keepends=True),
    result.stdout.splitlines(keepends=True),
    fromfile="expected",
    tofile="result",
)

print("".join(diff))

if result.stdout == expected:
    print("Everything ok")
    sys.exit(0)
else:
    print("Something is differnet :{")
    sys.exit(1)
