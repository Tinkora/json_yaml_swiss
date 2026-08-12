use json_yaml_swiss_core::{
    CONTRACT_VERSION, ConvertOptions, CoreError, Format, MAX_INPUT_BYTES, RootType, WarningCode,
    convert, detect, inspect, inspect_bytes,
};

fn error_code(result: Result<impl Sized, CoreError>) -> &'static str {
    match result {
        Ok(_) => panic!("expected an error"),
        Err(error) => error.code(),
    }
}

#[test]
fn inspect_reports_document_shape() {
    let report = inspect(Format::Json, r#"{"agent":{"tools":["read","write"]}}"#).unwrap();

    assert_eq!(report.version, CONTRACT_VERSION);
    assert_eq!(report.format, Format::Json);
    assert_eq!(report.root_type, RootType::Object);
    assert_eq!(report.node_count, 5);
    assert_eq!(report.max_depth, 4);
    assert_eq!(report.byte_size, 36);
}

#[test]
fn detection_reports_every_matching_parser() {
    let report = detect("true").unwrap();

    assert_eq!(report.candidates, vec![Format::Json, Format::Yaml]);
    assert_eq!(report.suggestion, Some(Format::Json));
    assert!(report.ambiguous);
}

#[test]
fn detection_distinguishes_toml_assignments() {
    let report = detect("name = \"tinkora\"").unwrap();

    assert_eq!(report.candidates, vec![Format::Toml, Format::Yaml]);
    assert_eq!(report.suggestion, Some(Format::Toml));
    assert!(report.ambiguous);
}

#[test]
fn duplicate_keys_are_never_overwritten() {
    assert_eq!(
        error_code(inspect(Format::Json, r#"{"name":1,"name":2}"#)),
        "DUPLICATE_KEY"
    );
    assert_eq!(
        error_code(inspect(Format::Yaml, "name: 1\nname: 2\n")),
        "DUPLICATE_KEY"
    );
    assert_eq!(
        error_code(inspect(Format::Toml, "name = 1\nname = 2\n")),
        "DUPLICATE_KEY"
    );
}

#[test]
fn yaml_tags_and_non_string_keys_are_rejected() {
    assert_eq!(
        error_code(inspect(Format::Yaml, "value: !secret token\n")),
        "UNSUPPORTED_YAML_TAG"
    );
    assert_eq!(
        error_code(inspect(Format::Yaml, "1: numeric-key\n")),
        "UNSUPPORTED_YAML_KEY"
    );
    assert_eq!(
        error_code(inspect(Format::Yaml, "!document\nvalue: token\n")),
        "UNSUPPORTED_YAML_TAG"
    );
    assert_eq!(
        error_code(inspect(Format::Yaml, "!key name: token\n")),
        "UNSUPPORTED_YAML_TAG"
    );
}

#[test]
fn yaml_merge_keys_are_preserved_as_literal_data() {
    let input = "defaults: &defaults\n  retries: 3\nagent:\n  <<: *defaults\n";

    let report = convert(
        Format::Yaml,
        Format::Json,
        input,
        &ConvertOptions::default(),
    )
    .unwrap();
    let value = serde_json::from_str::<serde_json::Value>(&report.output).unwrap();

    assert_eq!(value["agent"]["<<"]["retries"], 3);
    assert!(value["agent"].get("retries").is_none());

    let quoted = convert(
        Format::Yaml,
        Format::Json,
        "\"<<\": literal\n",
        &ConvertOptions::default(),
    )
    .unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&quoted.output).unwrap()["<<"],
        "literal"
    );
}

#[test]
fn yaml_streams_are_rejected() {
    assert_eq!(
        error_code(inspect(Format::Yaml, "---\na: 1\n---\nb: 2\n")),
        "MULTIPLE_YAML_DOCUMENTS"
    );
}

#[test]
fn toml_datetime_conversion_is_explicitly_lossy() {
    let report = convert(
        Format::Toml,
        Format::Json,
        "created = 2026-08-12T09:30:00Z\n",
        &ConvertOptions::default(),
    )
    .unwrap();

    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&report.output).unwrap()["created"],
        "2026-08-12T09:30:00Z"
    );
    assert!(
        report
            .warnings
            .contains(&WarningCode::TomlDatetimeStringified)
    );
}

#[test]
fn values_toml_cannot_represent_are_rejected() {
    assert_eq!(
        error_code(convert(
            Format::Json,
            Format::Toml,
            r#"{"missing":null}"#,
            &ConvertOptions::default(),
        )),
        "TARGET_CANNOT_REPRESENT_VALUE"
    );
    assert_eq!(
        error_code(convert(
            Format::Json,
            Format::Toml,
            "[1,2,3]",
            &ConvertOptions::default(),
        )),
        "TARGET_CANNOT_REPRESENT_VALUE"
    );
}

#[test]
fn non_finite_numbers_are_rejected() {
    for input in ["value = nan\n", "value = inf\n", "value = -inf\n"] {
        assert_eq!(
            error_code(inspect(Format::Toml, input)),
            "TARGET_CANNOT_REPRESENT_VALUE"
        );
    }
    for input in ["value: .nan\n", "value: .inf\n", "value: -.inf\n"] {
        assert_eq!(
            error_code(inspect(Format::Yaml, input)),
            "TARGET_CANNOT_REPRESENT_VALUE"
        );
    }
}

#[test]
fn every_cross_format_direction_preserves_the_common_subset() {
    let json = r#"{"agent":{"enabled":true,"retries":3,"ratio":0.5,"tools":["read","write"]}}"#;
    let yaml =
        "agent:\n  enabled: true\n  ratio: 0.5\n  retries: 3\n  tools:\n    - read\n    - write\n";
    let toml = "[agent]\nenabled = true\nratio = 0.5\nretries = 3\ntools = [\"read\", \"write\"]\n";

    for (from, input) in [
        (Format::Json, json),
        (Format::Yaml, yaml),
        (Format::Toml, toml),
    ] {
        for to in Format::all() {
            if from == to {
                continue;
            }
            let converted = convert(from, to, input, &ConvertOptions::default()).unwrap();
            let normalized = convert(
                to,
                Format::Json,
                &converted.output,
                &ConvertOptions::default(),
            )
            .unwrap();
            assert_eq!(
                serde_json::from_str::<serde_json::Value>(&normalized.output).unwrap(),
                serde_json::from_str::<serde_json::Value>(json).unwrap(),
                "{from:?} to {to:?} changed the normalized value"
            );
        }
    }
}

#[test]
fn conversion_sorts_keys_and_reports_normalization() {
    let report = convert(
        Format::Json,
        Format::Json,
        r#"{"z":1,"a":2}"#,
        &ConvertOptions::default(),
    )
    .unwrap();

    assert!(report.output.find("\"a\"").unwrap() < report.output.find("\"z\"").unwrap());
    assert_eq!(report.warnings, vec![WarningCode::KeyOrderNormalized]);
}

#[test]
fn source_presentation_warnings_are_ordered() {
    let report = convert(
        Format::Yaml,
        Format::Json,
        "# local only\nname: tinkora\n",
        &ConvertOptions::default(),
    )
    .unwrap();

    assert_eq!(
        report.warnings,
        vec![
            WarningCode::CommentsNotPreserved,
            WarningCode::PresentationNotPreserved,
            WarningCode::KeyOrderNormalized,
        ]
    );
}

#[test]
fn input_size_limit_is_enforced_before_parsing() {
    let oversized = " ".repeat(MAX_INPUT_BYTES + 1);

    assert_eq!(
        error_code(inspect(Format::Json, &oversized)),
        "INPUT_TOO_LARGE"
    );
}

#[test]
fn node_limit_is_enforced() {
    let input = format!("[{}]", vec!["0"; 100_000].join(","));

    assert_eq!(
        error_code(inspect(Format::Json, &input)),
        "DOCUMENT_TOO_COMPLEX"
    );
}

#[test]
fn detection_enforces_the_node_limit() {
    let input = format!("[{}]", vec!["0"; 100_000].join(","));

    assert_eq!(error_code(detect(&input)), "DOCUMENT_TOO_COMPLEX");
}

#[test]
fn output_size_limit_is_enforced() {
    let input = format!(
        "{}[{}]{}",
        "[".repeat(100),
        vec!["0"; 50_000].join(","),
        "]".repeat(100)
    );

    assert_eq!(
        error_code(convert(
            Format::Json,
            Format::Json,
            &input,
            &ConvertOptions {
                pretty: true,
                indent: 8,
            },
        )),
        "OUTPUT_TOO_LARGE"
    );
}

#[test]
fn invalid_utf8_has_a_distinct_error() {
    assert_eq!(
        error_code(inspect_bytes(Format::Json, &[0xff, 0xfe])),
        "INVALID_UTF8"
    );
}

#[test]
fn depth_limit_is_enforced() {
    let input = format!("{}0{}", "[".repeat(129), "]".repeat(129));

    assert_eq!(
        error_code(inspect(Format::Json, &input)),
        "DOCUMENT_TOO_COMPLEX"
    );
}

#[test]
fn stable_errors_cover_the_public_contract() {
    assert_eq!(CoreError::EmptyInput.code(), "EMPTY_INPUT");
    assert_eq!(
        CoreError::InputTooLarge {
            actual: 10,
            limit: 5,
        }
        .code(),
        "INPUT_TOO_LARGE"
    );
    assert_eq!(CoreError::AmbiguousFormat.code(), "AMBIGUOUS_FORMAT");
    assert_eq!(CoreError::InvalidUtf8.code(), "INVALID_UTF8");
    assert_eq!(CoreError::OutputTooLarge.code(), "OUTPUT_TOO_LARGE");
    assert_eq!(CoreError::SerializationError.code(), "SERIALIZATION_ERROR");
}
