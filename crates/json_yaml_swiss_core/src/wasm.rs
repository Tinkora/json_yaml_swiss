use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::{ConvertOptions, CoreError, Format, convert, detect, inspect};

fn error_to_js(error: CoreError) -> JsValue {
    let object = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&object, &"code".into(), &error.code().into());
    let _ = js_sys::Reflect::set(&object, &"message".into(), &error.to_string().into());
    object.into()
}

fn report_to_js<T: Serialize>(report: &T) -> Result<JsValue, JsValue> {
    serde_wasm_bindgen::to_value(report).map_err(|_| error_to_js(CoreError::SerializationError))
}

/// Inspect a document using an explicitly selected source format.
#[wasm_bindgen]
pub fn wasm_inspect(format: &str, input: &str) -> Result<JsValue, JsValue> {
    let format = Format::parse(format).map_err(error_to_js)?;
    let report = inspect(format, input).map_err(error_to_js)?;
    report_to_js(&report)
}

/// Return every supported parser that accepts the input.
#[wasm_bindgen]
pub fn wasm_detect(input: &str) -> Result<JsValue, JsValue> {
    let report = detect(input).map_err(error_to_js)?;
    report_to_js(&report)
}

/// Convert a document and return the versioned report with warning codes.
#[wasm_bindgen]
pub fn wasm_convert(
    from: &str,
    to: &str,
    input: &str,
    pretty: bool,
    indent: u32,
) -> Result<JsValue, JsValue> {
    let from = Format::parse(from).map_err(error_to_js)?;
    let to = Format::parse(to).map_err(error_to_js)?;
    let options = ConvertOptions {
        pretty,
        indent: indent.clamp(1, 8) as usize,
    };
    let report = convert(from, to, input, &options).map_err(error_to_js)?;
    report_to_js(&report)
}

/// Return format identifiers in their stable detection order.
#[wasm_bindgen]
pub fn wasm_formats() -> Result<JsValue, JsValue> {
    report_to_js(&Format::all())
}
