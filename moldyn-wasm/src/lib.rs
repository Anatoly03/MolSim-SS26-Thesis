//! This library provides a WebAssembly interface for the molecular dynamics
//! simulation. Specifically it provides wrappers for the [moldyn_core] module
//! and exposes them to JavaScript using [mod@wasm_bindgen].
//!
//! > Currently Not Implemented.
//!
//! To read more about WebAssembly with Rust, read the [wasm-bindgen guide](https://wasm-bindgen.github.io/wasm-bindgen/introduction.html)

// #[macro_use]
// mod macros;
mod vec3;

pub use vec3::Vec3Wrapper;
use wasm_bindgen::prelude::*;

// TODO continue reading the web assembly book
// https://wasm-bindgen.github.io/wasm-bindgen/examples/console-log.html

#[wasm_bindgen]
extern "C" {
    /// Logs a string to the JavaScript console. This is a simple wrapper for the
    /// `console.log` function in JavaScript.
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    log(&format!("Hello, {name}!"));
}

// read:
// https://wasm-bindgen.github.io/wasm-bindgen/examples/console-log.html
// https://wasm-bindgen.github.io/wasm-bindgen/reference/attributes/on-rust-exports/skip_typescript.html

