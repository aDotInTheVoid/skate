#!/bin/bash
set -eux -o pipefail

rm -rf cov/data
export CARGO_TARGET_DIR="./cov/target"
RUSTFLAGS="-Zinstrument-coverage" cargo +nightly build
./test.py --coverage
llvm-profdata merge -sparse ./cov/data/profile.*.profraw -o cov/out.profdata

llvm-cov show   -Xdemangler=rustfilt cov/target/debug/skate -instr-profile  cov/out.profdata -show-line-counts-or-regions -show-instantiations -format=html -ignore-filename-regex="(.cargo)|(.rustup)" -line-coverage-lt=99 > cov/cov.html
llvm-cov report -Xdemangler=rustfilt cov/target/debug/skate -instr-profile  cov/out.profdata  -ignore-filename-regex="(.cargo)|(.rustup)" -line-coverage-lt=99 > cov.txt


rm default.profraw