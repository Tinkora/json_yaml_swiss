use std::collections::BTreeMap;
use std::fmt;

use serde::de::{self, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Serialize};

use crate::CoreError;

pub const CONTRACT_VERSION: u32 = 1;
pub const MAX_INPUT_BYTES: usize = 2 * 1024 * 1024;
pub const MAX_OUTPUT_BYTES: usize = 4 * 1024 * 1024;
pub const MAX_NODES: usize = 100_000;
pub const MAX_DEPTH: usize = 128;

/// Supported configuration formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Json,
    Yaml,
    Toml,
}

impl Format {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::Yaml => "yaml",
            Self::Toml => "toml",
        }
    }

    pub fn parse(value: &str) -> Result<Self, CoreError> {
        match value.to_ascii_lowercase().as_str() {
            "json" => Ok(Self::Json),
            "yaml" | "yml" => Ok(Self::Yaml),
            "toml" => Ok(Self::Toml),
            other => Err(CoreError::UnsupportedFormat(other.to_owned())),
        }
    }

    pub const fn all() -> [Self; 3] {
        [Self::Json, Self::Toml, Self::Yaml]
    }
}

/// Root value category reported by inspection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RootType {
    Null,
    Boolean,
    Integer,
    Float,
    String,
    Array,
    Object,
}

/// Stable warning codes returned in declaration order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WarningCode {
    CommentsNotPreserved,
    PresentationNotPreserved,
    KeyOrderNormalized,
    TomlDatetimeStringified,
}

/// Output controls that do not alter the normalized semantic value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConvertOptions {
    pub pretty: bool,
    pub indent: usize,
}

impl Default for ConvertOptions {
    fn default() -> Self {
        Self {
            pretty: true,
            indent: 2,
        }
    }
}

/// Structural information about a parsed document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectionReport {
    pub version: u32,
    pub format: Format,
    pub root_type: RootType,
    pub node_count: usize,
    pub max_depth: usize,
    pub byte_size: usize,
}

/// Advisory format detection result. Callers remain responsible for selection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DetectionReport {
    pub version: u32,
    pub candidates: Vec<Format>,
    pub suggestion: Option<Format>,
    pub ambiguous: bool,
}

/// Successful conversion output and every normalization warning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConversionReport {
    pub version: u32,
    pub from: Format,
    pub to: Format,
    pub output: String,
    pub warnings: Vec<WarningCode>,
}

#[derive(Debug, Clone, PartialEq)]
enum CommonValue {
    Null,
    Boolean(bool),
    Integer(i64),
    Unsigned(u64),
    Float(f64),
    String(String),
    Array(Vec<Self>),
    Object(BTreeMap<String, Self>),
    TomlDatetime(toml::value::Datetime),
}

impl CommonValue {
    fn root_type(&self) -> RootType {
        match self {
            Self::Null => RootType::Null,
            Self::Boolean(_) => RootType::Boolean,
            Self::Integer(_) | Self::Unsigned(_) => RootType::Integer,
            Self::Float(_) => RootType::Float,
            Self::String(_) | Self::TomlDatetime(_) => RootType::String,
            Self::Array(_) => RootType::Array,
            Self::Object(_) => RootType::Object,
        }
    }

    fn statistics(&self) -> Result<(usize, usize), CoreError> {
        let mut nodes = 0usize;
        let mut max_depth = 0usize;
        let mut stack = vec![(self, 1usize)];

        while let Some((value, depth)) = stack.pop() {
            nodes = nodes.checked_add(1).ok_or(CoreError::DocumentTooComplex)?;
            if nodes > MAX_NODES || depth > MAX_DEPTH {
                return Err(CoreError::DocumentTooComplex);
            }
            max_depth = max_depth.max(depth);

            match value {
                Self::Array(values) => {
                    stack.extend(values.iter().rev().map(|value| (value, depth + 1)));
                }
                Self::Object(values) => {
                    stack.extend(values.values().rev().map(|value| (value, depth + 1)));
                }
                _ => {}
            }
        }

        Ok((nodes, max_depth))
    }

    fn contains_object(&self) -> bool {
        match self {
            Self::Object(_) => true,
            Self::Array(values) => values.iter().any(Self::contains_object),
            _ => false,
        }
    }

    fn contains_datetime(&self) -> bool {
        match self {
            Self::TomlDatetime(_) => true,
            Self::Array(values) => values.iter().any(Self::contains_datetime),
            Self::Object(values) => values.values().any(Self::contains_datetime),
            _ => false,
        }
    }
}

impl<'de> Deserialize<'de> for CommonValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CommonValueVisitor;

        impl<'de> Visitor<'de> for CommonValueVisitor {
            type Value = CommonValue;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a JSON value")
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(CommonValue::Null)
            }

            fn visit_none<E>(self) -> Result<Self::Value, E> {
                Ok(CommonValue::Null)
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
                Ok(CommonValue::Boolean(value))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
                Ok(CommonValue::Integer(value))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
                Ok(CommonValue::Unsigned(value))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value.is_finite() {
                    Ok(CommonValue::Float(value))
                } else {
                    Err(E::custom("non-finite numbers are not supported"))
                }
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
                Ok(CommonValue::String(value.to_owned()))
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
                Ok(CommonValue::String(value))
            }

            fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = sequence.next_element()? {
                    values.push(value);
                }
                Ok(CommonValue::Array(values))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut values = BTreeMap::new();
                while let Some(key) = map.next_key::<String>()? {
                    if values.contains_key(&key) {
                        return Err(de::Error::custom(format!("duplicate key `{key}`")));
                    }
                    values.insert(key, map.next_value()?);
                }
                Ok(CommonValue::Object(values))
            }
        }

        deserializer.deserialize_any(CommonValueVisitor)
    }
}

/// Inspect a UTF-8 document after strict parsing and normalization checks.
pub fn inspect(format: Format, input: &str) -> Result<InspectionReport, CoreError> {
    check_input(input)?;
    let value = parse(format, input)?;
    let (node_count, max_depth) = value.statistics()?;

    Ok(InspectionReport {
        version: CONTRACT_VERSION,
        format,
        root_type: value.root_type(),
        node_count,
        max_depth,
        byte_size: input.len(),
    })
}

/// Inspect bytes while preserving a distinct invalid UTF-8 diagnostic.
pub fn inspect_bytes(format: Format, input: &[u8]) -> Result<InspectionReport, CoreError> {
    let input = std::str::from_utf8(input).map_err(|_| CoreError::InvalidUtf8)?;
    inspect(format, input)
}

/// Return all parsers that accept the input and a non-authoritative suggestion.
pub fn detect(input: &str) -> Result<DetectionReport, CoreError> {
    check_input(input)?;

    let mut candidates = Vec::new();
    for format in Format::all() {
        match parse(format, input) {
            Ok(value) => {
                value.statistics()?;
                candidates.push(format);
            }
            Err(CoreError::DocumentTooComplex) => {
                return Err(CoreError::DocumentTooComplex);
            }
            Err(_) => {}
        }
    }
    if candidates.is_empty() {
        return Err(CoreError::UnknownFormat);
    }

    let suggestion = if candidates.contains(&Format::Json) {
        Some(Format::Json)
    } else if candidates.contains(&Format::Toml) && looks_like_toml(input) {
        Some(Format::Toml)
    } else if candidates.len() == 1 {
        candidates.first().copied()
    } else {
        None
    };

    Ok(DetectionReport {
        version: CONTRACT_VERSION,
        ambiguous: candidates.len() > 1,
        candidates,
        suggestion,
    })
}

/// Convert a document when every normalized value is representable by the target.
pub fn convert(
    from: Format,
    to: Format,
    input: &str,
    options: &ConvertOptions,
) -> Result<ConversionReport, CoreError> {
    check_input(input)?;
    let value = parse(from, input)?;
    value.statistics()?;
    let warnings = warnings_for(from, to, input, &value);
    let output = serialize(to, &value, options)?;
    if output.len() > MAX_OUTPUT_BYTES {
        return Err(CoreError::OutputTooLarge);
    }

    Ok(ConversionReport {
        version: CONTRACT_VERSION,
        from,
        to,
        output,
        warnings,
    })
}

fn check_input(input: &str) -> Result<(), CoreError> {
    if input.len() > MAX_INPUT_BYTES {
        return Err(CoreError::InputTooLarge {
            actual: input.len(),
            limit: MAX_INPUT_BYTES,
        });
    }
    if input.trim().is_empty() {
        return Err(CoreError::EmptyInput);
    }
    Ok(())
}

fn parse(format: Format, input: &str) -> Result<CommonValue, CoreError> {
    match format {
        Format::Json => parse_json(input),
        Format::Yaml => parse_yaml(input),
        Format::Toml => parse_toml(input),
    }
}

fn parse_json(input: &str) -> Result<CommonValue, CoreError> {
    serde_json::from_str(input).map_err(|error| {
        let message = error.to_string();
        if message.contains("duplicate key") {
            CoreError::DuplicateKey(message)
        } else if message.contains("recursion limit exceeded") {
            CoreError::DocumentTooComplex
        } else {
            CoreError::InvalidJson(message)
        }
    })
}

fn parse_yaml(input: &str) -> Result<CommonValue, CoreError> {
    let value = serde_yaml::from_str::<serde_yaml::Value>(input).map_err(|error| {
        let message = error.to_string();
        if message.contains("more than one document") {
            CoreError::MultipleYamlDocuments
        } else if message.contains("duplicate entry") {
            CoreError::DuplicateKey(message)
        } else if message.contains("recursion limit exceeded") {
            CoreError::DocumentTooComplex
        } else {
            CoreError::InvalidYaml(message)
        }
    })?;
    yaml_to_common(value)
}

fn yaml_to_common(value: serde_yaml::Value) -> Result<CommonValue, CoreError> {
    match value {
        serde_yaml::Value::Null => Ok(CommonValue::Null),
        serde_yaml::Value::Bool(value) => Ok(CommonValue::Boolean(value)),
        serde_yaml::Value::Number(value) => {
            if let Some(value) = value.as_i64() {
                Ok(CommonValue::Integer(value))
            } else if let Some(value) = value.as_u64() {
                Ok(CommonValue::Unsigned(value))
            } else if let Some(value) = value.as_f64().filter(|value| value.is_finite()) {
                Ok(CommonValue::Float(value))
            } else {
                Err(CoreError::TargetCannotRepresentValue(
                    "Non-finite YAML numbers are not supported".to_owned(),
                ))
            }
        }
        serde_yaml::Value::String(value) => Ok(CommonValue::String(value)),
        serde_yaml::Value::Sequence(values) => values
            .into_iter()
            .map(yaml_to_common)
            .collect::<Result<Vec<_>, _>>()
            .map(CommonValue::Array),
        serde_yaml::Value::Mapping(values) => {
            let mut object = BTreeMap::new();
            for (key, value) in values {
                let key = match key {
                    serde_yaml::Value::String(key) => key,
                    serde_yaml::Value::Tagged(_) => {
                        return Err(CoreError::UnsupportedYamlTag);
                    }
                    _ => return Err(CoreError::UnsupportedYamlKey),
                };
                object.insert(key, yaml_to_common(value)?);
            }
            Ok(CommonValue::Object(object))
        }
        serde_yaml::Value::Tagged(_) => Err(CoreError::UnsupportedYamlTag),
    }
}

fn parse_toml(input: &str) -> Result<CommonValue, CoreError> {
    let value = toml::from_str::<toml::Value>(input).map_err(|error| {
        let message = error.to_string();
        if message.contains("duplicate key") {
            CoreError::DuplicateKey(message)
        } else if message.contains("recursion limit exceeded") {
            CoreError::DocumentTooComplex
        } else {
            CoreError::InvalidToml(message)
        }
    })?;
    toml_to_common(value)
}

fn toml_to_common(value: toml::Value) -> Result<CommonValue, CoreError> {
    match value {
        toml::Value::String(value) => Ok(CommonValue::String(value)),
        toml::Value::Integer(value) => Ok(CommonValue::Integer(value)),
        toml::Value::Float(value) if value.is_finite() => Ok(CommonValue::Float(value)),
        toml::Value::Float(_) => Err(CoreError::TargetCannotRepresentValue(
            "Non-finite TOML numbers are not supported".to_owned(),
        )),
        toml::Value::Boolean(value) => Ok(CommonValue::Boolean(value)),
        toml::Value::Datetime(value) => Ok(CommonValue::TomlDatetime(value)),
        toml::Value::Array(values) => values
            .into_iter()
            .map(toml_to_common)
            .collect::<Result<Vec<_>, _>>()
            .map(CommonValue::Array),
        toml::Value::Table(values) => values
            .into_iter()
            .map(|(key, value)| Ok((key, toml_to_common(value)?)))
            .collect::<Result<BTreeMap<_, _>, _>>()
            .map(CommonValue::Object),
    }
}

fn serialize(
    format: Format,
    value: &CommonValue,
    options: &ConvertOptions,
) -> Result<String, CoreError> {
    match format {
        Format::Json => serialize_json(value, options),
        Format::Yaml => serialize_yaml(value),
        Format::Toml => serialize_toml(value, options),
    }
}

fn serialize_json(value: &CommonValue, options: &ConvertOptions) -> Result<String, CoreError> {
    let value = common_to_json(value)?;
    if options.pretty {
        let indent = vec![b' '; options.indent.clamp(1, 8)];
        let formatter = serde_json::ser::PrettyFormatter::with_indent(&indent);
        let mut buffer = Vec::new();
        let mut serializer = serde_json::Serializer::with_formatter(&mut buffer, formatter);
        value
            .serialize(&mut serializer)
            .map_err(|_| CoreError::SerializationError)?;
        String::from_utf8(buffer).map_err(|_| CoreError::SerializationError)
    } else {
        serde_json::to_string(&value).map_err(|_| CoreError::SerializationError)
    }
}

fn common_to_json(value: &CommonValue) -> Result<serde_json::Value, CoreError> {
    match value {
        CommonValue::Null => Ok(serde_json::Value::Null),
        CommonValue::Boolean(value) => Ok(serde_json::Value::Bool(*value)),
        CommonValue::Integer(value) => Ok(serde_json::Value::Number((*value).into())),
        CommonValue::Unsigned(value) => Ok(serde_json::Value::Number((*value).into())),
        CommonValue::Float(value) => serde_json::Number::from_f64(*value)
            .map(serde_json::Value::Number)
            .ok_or(CoreError::SerializationError),
        CommonValue::String(value) => Ok(serde_json::Value::String(value.clone())),
        CommonValue::TomlDatetime(value) => Ok(serde_json::Value::String(value.to_string())),
        CommonValue::Array(values) => values
            .iter()
            .map(common_to_json)
            .collect::<Result<Vec<_>, _>>()
            .map(serde_json::Value::Array),
        CommonValue::Object(values) => values
            .iter()
            .map(|(key, value)| Ok((key.clone(), common_to_json(value)?)))
            .collect::<Result<serde_json::Map<_, _>, _>>()
            .map(serde_json::Value::Object),
    }
}

fn serialize_yaml(value: &CommonValue) -> Result<String, CoreError> {
    let value = common_to_yaml(value)?;
    serde_yaml::to_string(&value).map_err(|_| CoreError::SerializationError)
}

fn common_to_yaml(value: &CommonValue) -> Result<serde_yaml::Value, CoreError> {
    match value {
        CommonValue::Null => Ok(serde_yaml::Value::Null),
        CommonValue::Boolean(value) => Ok(serde_yaml::Value::Bool(*value)),
        CommonValue::Integer(value) => Ok(serde_yaml::Value::Number((*value).into())),
        CommonValue::Unsigned(value) => Ok(serde_yaml::Value::Number((*value).into())),
        CommonValue::Float(value) => Ok(serde_yaml::Value::Number((*value).into())),
        CommonValue::String(value) => Ok(serde_yaml::Value::String(value.clone())),
        CommonValue::TomlDatetime(value) => Ok(serde_yaml::Value::String(value.to_string())),
        CommonValue::Array(values) => values
            .iter()
            .map(common_to_yaml)
            .collect::<Result<Vec<_>, _>>()
            .map(serde_yaml::Value::Sequence),
        CommonValue::Object(values) => {
            let mut mapping = serde_yaml::Mapping::new();
            for (key, value) in values {
                mapping.insert(
                    serde_yaml::Value::String(key.clone()),
                    common_to_yaml(value)?,
                );
            }
            Ok(serde_yaml::Value::Mapping(mapping))
        }
    }
}

fn serialize_toml(value: &CommonValue, options: &ConvertOptions) -> Result<String, CoreError> {
    if !matches!(value, CommonValue::Object(_)) {
        return Err(CoreError::TargetCannotRepresentValue(
            "TOML output requires an object root".to_owned(),
        ));
    }
    let value = common_to_toml(value)?;
    if options.pretty {
        toml::to_string_pretty(&value).map_err(|_| CoreError::SerializationError)
    } else {
        toml::to_string(&value).map_err(|_| CoreError::SerializationError)
    }
}

fn common_to_toml(value: &CommonValue) -> Result<toml::Value, CoreError> {
    match value {
        CommonValue::Null => Err(CoreError::TargetCannotRepresentValue(
            "TOML cannot represent null".to_owned(),
        )),
        CommonValue::Boolean(value) => Ok(toml::Value::Boolean(*value)),
        CommonValue::Integer(value) => Ok(toml::Value::Integer(*value)),
        CommonValue::Unsigned(value) => {
            i64::try_from(*value)
                .map(toml::Value::Integer)
                .map_err(|_| {
                    CoreError::TargetCannotRepresentValue(
                        "TOML integers cannot exceed i64::MAX".to_owned(),
                    )
                })
        }
        CommonValue::Float(value) => Ok(toml::Value::Float(*value)),
        CommonValue::String(value) => Ok(toml::Value::String(value.clone())),
        CommonValue::TomlDatetime(value) => Ok(toml::Value::Datetime(*value)),
        CommonValue::Array(values) => values
            .iter()
            .map(common_to_toml)
            .collect::<Result<Vec<_>, _>>()
            .map(toml::Value::Array),
        CommonValue::Object(values) => values
            .iter()
            .map(|(key, value)| Ok((key.clone(), common_to_toml(value)?)))
            .collect::<Result<toml::Table, _>>()
            .map(toml::Value::Table),
    }
}

fn warnings_for(from: Format, to: Format, input: &str, value: &CommonValue) -> Vec<WarningCode> {
    let mut warnings = Vec::new();
    if matches!(from, Format::Yaml | Format::Toml) && contains_comment(input) {
        warnings.push(WarningCode::CommentsNotPreserved);
    }
    if matches!(from, Format::Yaml | Format::Toml) {
        warnings.push(WarningCode::PresentationNotPreserved);
    }
    if value.contains_object() {
        warnings.push(WarningCode::KeyOrderNormalized);
    }
    if from == Format::Toml && to != Format::Toml && value.contains_datetime() {
        warnings.push(WarningCode::TomlDatetimeStringified);
    }
    warnings
}

fn contains_comment(input: &str) -> bool {
    input.lines().any(|line| {
        let mut single_quoted = false;
        let mut double_quoted = false;
        let mut escaped = false;
        for character in line.chars() {
            if escaped {
                escaped = false;
                continue;
            }
            if character == '\\' && double_quoted {
                escaped = true;
            } else if character == '\'' && !double_quoted {
                single_quoted = !single_quoted;
            } else if character == '"' && !single_quoted {
                double_quoted = !double_quoted;
            } else if character == '#' && !single_quoted && !double_quoted {
                return true;
            }
        }
        false
    })
}

fn looks_like_toml(input: &str) -> bool {
    input.lines().any(|line| {
        let line = line.trim();
        (!line.starts_with('#') && line.contains('='))
            || (line.starts_with('[') && line.ends_with(']'))
    })
}
