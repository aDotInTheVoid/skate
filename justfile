wasm:
    wasm-pack build --target web --out-name wasm --out-dir ./static --profiling ./playground

serve: wasm
    miniserve --index index.html ./playground/static
