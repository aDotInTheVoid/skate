#!/bin/bash
set -euo pipefail

sd -s "#[cfg_attr(rustfmt, rustfmt_skip)]" "" src/grammar.rs
sd -s "#[cfg_attr(rustfmt, rustfmt_skip)]" "" crates/parser/src/grammar.rs
cargo fmt

# https://unix.stackexchange.com/a/161853
# TODO: This doesnt fmt unstaged files, which is a real pain
git ls-files -z | while IFS= read -rd '' f 
do
    if [[ "${f: -7}" != ".stderr" &&  "${f: -7}" != ".stdout" ]]
    then
        tail -c1 < "$f" | read -r _ || echo >> "$f"
    fi
done
