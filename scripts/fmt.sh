#!/bin/bash
set -euo pipefail

cargo fmt

# https://unix.stackexchange.com/a/161853
# TODO: This doesnt fmt unstaged files, which is a real pain
# TODO: This trys to add to deleted files
git ls-files -z | while IFS= read -rd '' f 
do
    if [[ "${f: -7}" != ".stderr" &&  "${f: -7}" != ".stdout" && -f $f ]]
    then
        tail -c1 < "$f" | read -r _ || echo >> "$f"
    fi
done
