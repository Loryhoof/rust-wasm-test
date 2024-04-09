mod utils;
use utils::{Vector2, Vector3};

use wasm_bindgen::prelude::*;

fn start() {
    let vec1: Vector3 = Vector3::new(1.2, 1.5, 0.2);
}

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