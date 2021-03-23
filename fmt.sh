#!/bin/sh
sd -s "#[cfg_attr(rustfmt, rustfmt_skip)]" "" src/grammar.rs
cargo fmt