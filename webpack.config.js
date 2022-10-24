// This is the file to config webpack. Webpack are a set of tools to bundle, concatening and optimizing web application. 

const path = require('path');
const HTMLWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')

module.exports = {
    entry: './public/main.js',
    output: {
        path: path.resolve(__dirname, 'dist'), // need to be absolute path
        filename: 'index.js',
    },
    plugins: [
        new HTMLWebpackPlugin({
            template: './public/index.html', // need to be relative file
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, ".") // must point to the (absolute) root path of the Rust Project, searching for Cargo.toml file
        })
    ],
    experiments: {
        asyncWebAssembly: true,
    }
}