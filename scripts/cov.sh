#!/bin/bash
set -euxo pipefail

rm -rf cov/data
export CARGO_TARGET_DIR="./cov/target"
RUSTFLAGS="-Zinstrument-coverage" cargo +nightly build --all
RUSTFLAGS="-Zinstrument-coverage" LLVM_PROFILE_FILE="./cov/data/profile.%p.profraw" cargo +nightly test --all
./scripts/test.py --coverage
llvm-profdata merge -sparse ./cov/data/profile.*.profraw -o cov/out.profdata

llvm-cov show   -Xdemangler=rustfilt cov/target/debug/skate -instr-profile  cov/out.profdata -show-line-counts-or-regions -format=html  -ignore-filename-regex="(.cargo)|(.rustup)|(grammar.rs)" -show-instantiations=false -line-coverage-lt=99 > cov/cov.html
# llvm-cov show   -Xdemangler=rustfilt cov/target/debug/skate -instr-profile  cov/out.profdata -show-line-counts-or-regions  -format=html -ignore-filename-regex="(.cargo)|(.rustup)|(grammar.rs)" -show-instantiations=false > cov/cov.html

if [[ -r default.profraw ]]
then
    rm default.profraw
fi
