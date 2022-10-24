# Commands

`npm run serve`: run the webpack server so that changes will be included in the webpack bundle realtime. 


# Steps to build this example

1. Init a Web Project with a rust library.
2. Install and config webpack for js
3. Encode a uploaded image with base64:
3.1 so that we can pass the image from js to Rust. 
4. config webpack to compile Rust

4.1 ```npm install -D @wasm-tool/wasm-pack-plugin```

4.2 in `webpack.config.js` add an new entry of WasmPackPlugin to plugins

4.3 in `Cargo.toml`, add the `lib` table and the `crate-type` entry. "Crate" is "Library" or "Package" in other language.

4.4 add the `wasm-bindgen` crate in `Cargo.toml` (https://crates.io/crates)

4.5 in `main.js` load the `rustApp`, our wasm module

4.6 in `webpack.config.js`, flag the `pkg` module as WebAssembly module
5. Build the rust module to do the grayscale
expose js function to rust
convert a js val to rust val with `into()`

