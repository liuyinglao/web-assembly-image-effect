# Commands

`npm run serve`: run the webpack server so that changes will be included in the webpack bundle realtime. 


# Steps to build this example

1. Init a Web Project with a rust library.
2. Install and config webpack for js
3. Encode a uploaded image with base64:
    1. so that we can pass the image from js to Rust. 
4. config webpack to compile Rust
    1. ```npm install -D @wasm-tool/wasm-pack-plugin```
    2. in `webpack.config.js` add an new entry of WasmPackPlugin to plugins
    3. in `Cargo.toml`, add the `lib` table and the `crate-type` entry. "Crate" is "Library" or "Package" in other language.
    4. add the `wasm-bindgen` crate in `Cargo.toml` (https://crates.io/crates)
    5. in `main.js` load the `rustApp`, our wasm module
    6. in `webpack.config.js`, flag the `pkg` module as WebAssembly module
5. Build the rust module to do the grayscale
    1. expose js function to rust
    2. convert a js val to rust val with `into()`
6. Decode Base64
    1. add crate "image" and "base64" in `Cargo.toml`
    2. decode the encoded string and unwrap to get the image in type `Vec<u8>`
    3. `image::load_from_memory` to get `DynamicImage`
    4. apply `img.grayscale()`
    5. `write_to` a buffer of type `Vec<u8>`
    6. `encode` the `buffer` in `String`
    7. add the header back
    8. render the image in js
7. Deploy
    1. `npm run build`
    2. push to github 



