use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log; // alias
#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) {
    log(&encoded_file.into())
}
