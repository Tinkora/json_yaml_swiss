use thiserror::Error;

/// Stable error type shared by the Rust and WASM interfaces.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum CoreError {
    #[error("Input is empty")]
    EmptyInput,
    #[error("Input is {actual} bytes; the limit is {limit} bytes")]
    InputTooLarge { actual: usize, limit: usize },
    #[error("No supported parser accepts the input")]
    UnknownFormat,
    #[error("More than one supported format accepts the input")]
    AmbiguousFormat,
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("Input is not valid UTF-8")]
    InvalidUtf8,
    #[error("Invalid JSON: {0}")]
    InvalidJson(String),
    #[error("Invalid YAML: {0}")]
    InvalidYaml(String),
    #[error("Invalid TOML: {0}")]
    InvalidToml(String),
    #[error("The document contains duplicate key: {0}")]
    DuplicateKey(String),
    #[error("YAML streams with multiple documents are not supported")]
    MultipleYamlDocuments,
    #[error("YAML mapping keys must be strings")]
    UnsupportedYamlKey,
    #[error("YAML tags are not supported")]
    UnsupportedYamlTag,
    #[error("The document exceeds the supported depth or node-count limit")]
    DocumentTooComplex,
    #[error("{0}")]
    TargetCannotRepresentValue(String),
    #[error("Generated output exceeds the 4 MiB limit")]
    OutputTooLarge,
    #[error("Could not serialize the normalized document")]
    SerializationError,
}

impl CoreError {
    /// Returns the stable machine-readable code for this error.
    pub const fn code(&self) -> &'static str {
        match self {
            Self::EmptyInput => "EMPTY_INPUT",
            Self::InputTooLarge { .. } => "INPUT_TOO_LARGE",
            Self::UnknownFormat => "UNKNOWN_FORMAT",
            Self::AmbiguousFormat => "AMBIGUOUS_FORMAT",
            Self::UnsupportedFormat(_) => "UNSUPPORTED_FORMAT",
            Self::InvalidUtf8 => "INVALID_UTF8",
            Self::InvalidJson(_) => "INVALID_JSON",
            Self::InvalidYaml(_) => "INVALID_YAML",
            Self::InvalidToml(_) => "INVALID_TOML",
            Self::DuplicateKey(_) => "DUPLICATE_KEY",
            Self::MultipleYamlDocuments => "MULTIPLE_YAML_DOCUMENTS",
            Self::UnsupportedYamlKey => "UNSUPPORTED_YAML_KEY",
            Self::UnsupportedYamlTag => "UNSUPPORTED_YAML_TAG",
            Self::DocumentTooComplex => "DOCUMENT_TOO_COMPLEX",
            Self::TargetCannotRepresentValue(_) => "TARGET_CANNOT_REPRESENT_VALUE",
            Self::OutputTooLarge => "OUTPUT_TOO_LARGE",
            Self::SerializationError => "SERIALIZATION_ERROR",
        }
    }
}
