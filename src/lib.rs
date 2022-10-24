use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log; // alias
use base64::{decode, encode};
use image::load_from_memory;
use image::ImageOutputFormat::Png;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    // if you are borrowing a string, use str
    // if the string has ownership, use String
    log(&"Grayscale called".into());

    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image Decoded".into());

    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded".into());

    img = img.grayscale();
    log(&"Grayscaled".into());

    let mut buffer = Vec::new();
    img.write_to(&mut buffer, Png).unwrap();
    log(&"image written".into());

    let encoded_img = encode(&buffer);

    // convert string to data url
    let data_url = format!(
        "data:image/png;base64,{}", encoded_img
    );

    return data_url;

}
