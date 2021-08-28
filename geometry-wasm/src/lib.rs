use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test() -> String {
    return String::from("hello world");
}