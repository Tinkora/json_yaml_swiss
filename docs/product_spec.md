# JSON YAML Swiss Product Specification

[简体中文](product_spec.zh-CN.md)

Status: Alpha. Last reviewed: 2026-08-13.

## Objective

JSON YAML Swiss is a local browser tool for inspecting, validating, formatting,
and deliberately converting JSON, YAML, and TOML configuration data. It serves
developers who need to review sensitive configuration without pasting it into a
hosted converter.

The tool is not a lossless source-to-source editor. Configuration formats have
different data models, and parsing plus serialization can remove comments,
anchors, aliases, scalar style, whitespace, and key order. Every operation must
either preserve the supported semantic value, report a specific normalization,
or fail. It must never silently invent a key, coerce an unsupported value, or
overwrite a duplicate mapping entry.

## MVP Workflows

1. Paste or load a UTF-8 JSON, YAML, or TOML document.
2. Select the source format explicitly, or request a non-authoritative format
   suggestion that lists every matching parser.
3. Validate syntax and inspect format, root type, node count, maximum depth,
   byte size, and conversion diagnostics.
4. Format JSON, YAML, or TOML into deterministic normalized output.
5. Convert between JSON, YAML, and TOML only when the normalized value can be
   represented by the target format.
6. Copy or download the output after reviewing warnings.

## Data Contract

### Limits

- Input must be UTF-8 and no larger than 2 MiB.
- A normalized document may contain at most 100,000 values.
- Maximum nesting depth is 128.
- Generated output may not exceed 4 MiB.
- YAML input must contain exactly one document.

### Supported semantic values

The shared conversion model supports null, booleans, signed and unsigned
64-bit integers, finite floating-point values, strings, arrays, and objects with
unique string keys.

The following boundaries are explicit:

- YAML tags and non-string mapping keys are rejected for formatting and
  conversion.
- Duplicate keys are rejected by the selected parsers; they are never resolved
  using first-wins or last-wins behavior.
- TOML datetime values become strings only when converting away from TOML, and
  the result includes `TOML_DATETIME_STRINGIFIED`.
- JSON or YAML null values cannot be represented in TOML and are rejected with
  `TARGET_CANNOT_REPRESENT_VALUE`.
- TOML output requires an object at the document root.
- Non-finite floats are rejected.

### Normalization warnings

Successful formatting and conversion return a versioned report containing the
output and ordered warning codes. Warnings describe transformations rather than
claiming source fidelity:

- `COMMENTS_NOT_PRESERVED`
- `PRESENTATION_NOT_PRESERVED`
- `KEY_ORDER_NORMALIZED`
- `TOML_DATETIME_STRINGIFIED`

Comments and presentation warnings apply to YAML and TOML serialization. Key
order is normalized deterministically for every object or table.

### Format suggestion

Detection is advisory. The result contains all formats whose parser accepts the
input, an optional suggestion, and an `ambiguous` flag. JSON takes suggestion
precedence when valid because JSON is also valid YAML. TOML is suggested only
when its parser accepts the input and the text contains a TOML-specific key,
assignment, or table marker. The user-visible source selection remains explicit.

## Privacy And Security

- Parsing, validation, formatting, and conversion run in local Rust/WASM.
- The application makes no runtime network requests after its static files load.
- User input is not persisted in cookies, IndexedDB, `localStorage`, or service
  worker caches.
- User-controlled text is rendered through form values or `textContent`, never
  `innerHTML`.
- Clipboard and file download operations happen only after an explicit user
  action.
- No JavaScript parser fallback is permitted.

## Public Interfaces

The Rust core is authoritative. The WASM adapter returns serialized versions of
the same reports and stable errors. The MVP does not claim an MCP server,
Agent Skill, remote API, or hosted conversion service.

Stable error codes:

- `EMPTY_INPUT`
- `INPUT_TOO_LARGE`
- `UNKNOWN_FORMAT`
- `AMBIGUOUS_FORMAT`
- `UNSUPPORTED_FORMAT`
- `INVALID_UTF8`
- `INVALID_JSON`
- `INVALID_YAML`
- `INVALID_TOML`
- `DUPLICATE_KEY`
- `MULTIPLE_YAML_DOCUMENTS`
- `UNSUPPORTED_YAML_KEY`
- `UNSUPPORTED_YAML_TAG`
- `DOCUMENT_TOO_COMPLEX`
- `TARGET_CANNOT_REPRESENT_VALUE`
- `OUTPUT_TOO_LARGE`
- `SERIALIZATION_ERROR`

## Non-Goals

- Preserving comments, anchors, aliases, exact key order, whitespace, quoting,
  or scalar style.
- Schema validation, JSON Schema repair, merge-key evaluation, or template
  expansion.
- YAML multi-document streams.
- HCL, XML, INI, CSV, environment files, or arbitrary plugin formats.
- Executing configuration, resolving file references, or contacting remote URLs.
- Claiming semantic equivalence for values that require a warning or rejection.

## Verification

- Parser and conversion fixtures cover all six cross-format directions,
  formatting, ambiguous detection, duplicate keys, YAML tags/non-string keys,
  TOML datetimes, null-to-TOML rejection, limits, and stable diagnostics.
- Rust formatting, tests, Clippy, MSRV, and WASM checks pass from a clean checkout.
- Real Chromium tests exercise local files, validation, warnings, conversion,
  copy/download state, adversarial text rendering, and 375, 768, 1024, and 1440
  pixel viewports.
- Documentation tests enforce the required bilingual files, local links,
  UTF-8 encoding, and removal of obsolete public scaffolds.
- Hosted Quality, Documentation quality, Supply chain, CodeQL, and Pages runs
  pass for the exact public commit before an Alpha release.
