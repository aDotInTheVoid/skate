#!/bin/sh
sd -s "#[cfg_attr(rustfmt, rustfmt_skip)]" "" src/grammar.rs
cargo fmt

# https://unix.stackexchange.com/a/161853
git ls-files -z | while IFS= read -rd '' f; do tail -c1 < "$f" | read -r _ || echo >> "$f"; done

