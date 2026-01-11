#!/bin/bash

cloc $(find $(git ls-files) -type f ! -lname '*') --read-lang-def=clocdef

