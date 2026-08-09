# Repository Guide for AI Agents

## Project Overview

json_yaml_swiss is a browser-first universal config format converter. Convert between JSON, YAML, and TOML — plus pretty-print, minify, and validate — all in WASM. Privacy-first: zero server-side data.

## Architecture

```
json_yaml_swiss/
├── crates/
│   ├── json_yaml_swiss_core/       # Format detection, conversion engine, validation
│   └── json_yaml_swiss_web/        # WASM cdylib entry point + static HTML editor
├── docs/                            # Product spec, architecture, plans
├── skills/                          # Agent Skill definitions (MCP tools)
└── index.html                       # Product landing page
```

## Key Files for AI Context

| File | Purpose |
|------|---------|
| `crates/json_yaml_swiss_core/src/convert.rs` | Format detection, all conversion logic, pretty/minify/validate |
| `crates/json_yaml_swiss_core/src/error.rs` | Stable error type with machine-readable codes |
| `crates/json_yaml_swiss_core/src/wasm.rs` | WASM bindings (6 JS exports) |
| `crates/json_yaml_swiss_core/src/lib.rs` | Crate root, conditional WASM compilation |
| `crates/json_yaml_swiss_web/src/lib.rs` | cdylib entry point, re-exports |
| `crates/json_yaml_swiss_web/static/index.html` | Full-featured converter UI |
| `skills/json_yaml_swiss.md` | Agent usage workflow |
| `skills/mcp-tools.json` | MCP tool definitions |

## Build & Test Commands

```bash
# Run all tests
cargo test --workspace

# Format check
cargo fmt --all -- --check

# Lint (strict)
cargo clippy --workspace --all-targets -- -D warnings

# WASM compilation check
cargo check -p json_yaml_swiss_web --target wasm32-unknown-unknown

# Build Web WASM for deployment
wasm-pack build --target web crates/json_yaml_swiss_web
```

## Design Principles

1. **Browser-first**: All format conversion happens in-browser via WASM
2. **Privacy-first**: Zero data leaves the browser. No server, no telemetry, no tracking.
3. **Auto-detection**: Format detection heuristics: JSON (strictest) first, then TOML, then YAML (most permissive)
4. **Common IR**: All formats deserialize through serde into a common intermediate representation, then re-serialize to target
5. **Stable error codes**: Every CoreError has a machine-readable `code()` for programmatic consumers

## Format Detection Algorithm

1. Try JSON first (strictest syntax, most common)
2. If not JSON, try TOML (distinctive `[section]` / `key = value` syntax)
3. If not TOML, try YAML (most permissive, catch-all)
4. If nothing works, return `UnknownFormat` error

## Supported Conversions

All 6 directions: JSON↔YAML, JSON↔TOML, YAML↔TOML. Plus identity conversions (pretty/minify).

## Error Codes (Stable Machine-Readable)

| Code | Meaning |
|------|---------|
| `EMPTY_INPUT` | Input string is empty or whitespace-only |
| `UNKNOWN_FORMAT` | Cannot detect format from input |
| `UNSUPPORTED_FORMAT` | Format string not in {json, yaml, toml} |
| `INVALID_JSON` | JSON parsing failed |
| `INVALID_YAML` | YAML parsing failed |
| `INVALID_TOML` | TOML parsing failed |
| `PARSE_ERROR` | Generic parse failure |
| `CONVERSION_ERROR` | Conversion between formats failed |

## WASM Exports

| Function | Signature | Returns |
|----------|-----------|---------|
| `wasm_convert` | (from, to, input, pretty, indent) | Result<String, JsValue> |
| `wasm_detect_format` | (input) | Result<String, JsValue> |
| `wasm_pretty_print` | (format, input, indent) | Result<String, JsValue> |
| `wasm_minify` | (format, input) | Result<String, JsValue> |
| `wasm_validate` | (format, input) | {valid: bool, error: string|null} |
| `wasm_list_formats` | () | ["json","yaml","toml"] |

## Frontend Design Requirement

- Before creating, modifying, reviewing, or debugging any HTML page or user-facing frontend, invoke the `ui-ux-pro-max` skill.
- Run the skill's required `--design-system` search before editing, followed by relevant stack and UX searches.
- If `ui-ux-pro-max` is unavailable, stop frontend work and report the missing prerequisite.
- Verify the rendered result in a real browser at 375, 768, 1024, and 1440 pixel widths, including console, keyboard, accessibility, and overflow checks.
