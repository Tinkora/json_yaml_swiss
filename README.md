# JSON YAML Swiss

Inspect, validate, format, and deliberately convert JSON, YAML, and TOML in
your browser. Parsing runs locally in Rust/WebAssembly; configuration text is
not uploaded, stored, or sent to telemetry.

[简体中文](README.zh-CN.md)

[![Support Tinkora on Ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/tinkora)

## Why this exists

Configuration conversion is not inherently lossless. JSON, YAML, and TOML
have different value models and source features. Many converters silently
discard comments, overwrite duplicate keys, or coerce unsupported values.
JSON YAML Swiss keeps the boundary explicit: it reports normalization, rejects
values the target cannot represent, and never changes the selected source
format based only on detection.

Use it when a configuration file is sensitive, a local CLI is unavailable, or
you need to inspect the document shape before sharing it with a script or an AI
agent.

## What it does

- Inspect a selected JSON, YAML, or TOML document and report its root type,
  node count, depth, and byte size.
- Suggest every format whose parser accepts the input while preserving
  ambiguity.
- Normalize one format or convert across all six cross-format directions.
- Reject duplicate keys, unsupported YAML constructs, non-finite numbers, and
  values the target format cannot represent.
- Surface deterministic warning codes for presentation, comments, key order,
  and TOML datetime normalization.
- Copy or download the result from an English-first interface with a Chinese
  language switch.

## Conversion contract

The shared value model supports null, booleans, signed and unsigned 64-bit
integers, finite floating-point numbers, strings, arrays, and objects with
unique string keys.

The tool does not preserve comments, whitespace, quoting, scalar style,
anchors, aliases, or exact key order. YAML tags, non-string mapping keys, and
multi-document streams are rejected. YAML merge keys are preserved as literal
`<<` entries and are never evaluated. TOML datetimes become strings with a
warning when converted to JSON or YAML. TOML output rejects null and non-object
roots.

Limits are 2 MiB input, 100,000 normalized nodes, 128 levels of nesting, and
4 MiB output. See the [product specification](docs/product_spec.md) for the
complete contract and stable diagnostic codes.

## Browser tool

The public application is available at
<https://tinkora.github.io/json_yaml_swiss/>.

To run it locally:

```console
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --version 0.15.0 --locked
cd crates/json_yaml_swiss_web
npm ci --ignore-scripts
npm run build:wasm
python3 -m http.server 8080
```

Open <http://127.0.0.1:8080/static/>.

## Development checks

```console
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo check --workspace --target wasm32-unknown-unknown --locked
cd crates/json_yaml_swiss_web
npm ci --ignore-scripts
npx --no-install playwright install chromium
npm run test:browser
```

The browser suite builds the real release WASM package and exercises Chromium
at 375, 768, 1024, and 1440 pixels. It also checks local file input, stable
errors, adversarial text rendering, keyboard access, localization, clipboard
output, downloads, console errors, and unexpected external requests.

## Project status

The project is **Alpha**. Native, WASM, browser, documentation, dependency, and
security checks pass in hosted CI, and the deployed Pages application has been
verified. See [maturity](docs/MATURITY.md),
[contributing](CONTRIBUTING.md), [security](SECURITY.md),
[support](SUPPORT.md), and the [release checklist](docs/RELEASE_CHECKLIST.md).

## License and support

Released under the [MIT License](LICENSE).

If this tool saves you time, you can support Tinkora on Ko-fi. Tips are optional
and never a condition of use, support, or contribution.
