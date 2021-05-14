#!/bin/bash
set -euxo pipefail
curl -OL https://github.com/casey/just/releases/download/v0.9.2/just-v0.9.2-x86_64-unknown-linux-musl.tar.gz
curl -OL https://github.com/rustwasm/wasm-pack/releases/download/v0.9.1/wasm-pack-v0.9.1-x86_64-unknown-linux-musl.tar.gz
tar xf wasm-pack-v0.9.1-x86_64-unknown-linux-musl.tar.gz
tar xf just-v0.9.2-x86_64-unknown-linux-musl.tar.gz
mkdir bin
mv wasm-pack-v0.9.1-x86_64-unknown-linux-musl/wasm-pack bin
mv just bin
echo "$(pwd)/bin" >> $GITHUB_PATH