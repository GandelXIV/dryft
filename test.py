#!/usr/bin/python3
import subprocess
import difflib

result = subprocess.run(
    ["./a.out"],
    capture_output=True,
    text=True
)

expected = """
Hello World
(6/2)(1+2) = 9
6/(2(1+2)) = 1
2
6
1 0
0
1
1
0
equal!
nested conditionals work
not same
 0 1 2 3 4 5 6 7 8 9
"""[1:] # remove blank line

diff = difflib.unified_diff(
    expected.splitlines(keepends=True),
    result.stdout.splitlines(keepends=True),
    fromfile="expected",
    tofile="result",
)

print("".join(diff))

if result.stdout == expected:
    print("Everything ok")
else:
    print("Something is differnet :{")
