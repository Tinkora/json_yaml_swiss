//! WASM cdylib entry point for json_yaml_swiss.
//!
//! This crate wraps `json_yaml_swiss_core` and exposes its WASM bindings
//! as a cdylib that `wasm-pack` can build.

// Re-export all wasm-bindgen functions from core so they appear in the final .wasm.
pub use json_yaml_swiss_core::*;

/// Initialization function called by the JS glue code.
/// Sets up the console error panic hook for better debugging.
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}
