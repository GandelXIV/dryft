#!/bin/bash

# cloc cant handle symlinks, so we have to filter for it
cloc $(find $(git ls-files) -type f ! -lname '*') --read-lang-def=clocdef

