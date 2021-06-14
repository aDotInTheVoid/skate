wasm:
    wasm-pack build --target web --out-name wasm --out-dir ./static --profiling ./playground

serve: wasm
    miniserve --index index.html ./playground/static

build:
    cargo build

test-unit: build
    cargo test

test-e2e: build
    python3 ./scripts/test.py

bless: build
    python3 ./scripts/test.py --bless

test: test-unit test-e2e

fmt:
    ./scripts/fmt.sh

cov:
    ./scripts/cov.sh
