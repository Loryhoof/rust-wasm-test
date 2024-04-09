use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    finish_add(a, b)
}

fn finish_add(a: i32, b: i32) -> i32 {
    return a + b;
}

#[wasm_bindgen]
pub fn multiply(a: i32, b: i32) -> i32 {
    return a * b;
}