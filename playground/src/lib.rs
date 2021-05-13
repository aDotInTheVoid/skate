use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_code(code: &str) {
    skate::run(code, 1).unwrap();
}
