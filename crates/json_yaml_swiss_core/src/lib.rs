mod convert;
mod error;

#[cfg(target_arch = "wasm32")]
mod wasm;

pub use convert::{
    CONTRACT_VERSION, ConversionReport, ConvertOptions, DetectionReport, Format, InspectionReport,
    MAX_DEPTH, MAX_INPUT_BYTES, MAX_NODES, MAX_OUTPUT_BYTES, RootType, WarningCode, convert,
    detect, inspect, inspect_bytes,
};
pub use error::CoreError;

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
