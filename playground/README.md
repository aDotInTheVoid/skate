# Skate playground

```sh
wasm-pack build --target web --out-name wasm --out-dir ./static --dev
miniserve ./static --index index.html
```

```js
mport("./wasm.js").then(m => mod = m)
mod.run_code("fn main() { print \"hello\"}");
```
