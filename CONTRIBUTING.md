# Contributing

Thank you for improving JSON YAML Swiss. Keep contributions tied to a
reproducible configuration problem and prefer the smallest change that solves
it without weakening the conversion contract.

[简体中文](CONTRIBUTING.zh-CN.md)

## Before opening a change

- Search existing issues and discussions.
- For a feature, describe the user pain and existing alternatives before the
  implementation.
- Do not include secrets or private configuration in issues, fixtures, logs, or
  screenshots.
- Use GitHub Private Vulnerability Reporting for security findings.

## Development setup

Install Rust 1.95 or newer, the `wasm32-unknown-unknown` target, Node.js 24,
Python 3, and `wasm-pack 0.15.0`.

```console
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --version 0.15.0 --locked
cd crates/json_yaml_swiss_web
npm ci --ignore-scripts
npx --no-install playwright install chromium
```

Run the complete local gate before requesting review:

```console
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo check --workspace --target wasm32-unknown-unknown --locked
cd crates/json_yaml_swiss_web
npm audit --audit-level=high
npm run test:browser
```

## Change rules

- Rust core behavior is authoritative; the browser must call the real WASM
  exports without a second parser implementation.
- Reject unsupported values instead of silently coercing or overwriting data.
- Update outcome-focused tests for behavior changes, including invalid input
  and affected boundaries.
- Keep public documentation English-first and update the Chinese peer when
  user-visible meaning changes.
- Write code comments and commit messages in English. Commits use Conventional
  Commit prefixes such as `feat:`, `fix:`, `docs:`, `test:`, and `chore:`.
- Before changing HTML, CSS, or user-facing browser behavior, run the
  `ui-ux-pro-max` skill and verify 375, 768, 1024, and 1440 pixel viewports.

## Pull requests

Keep each pull request focused. Explain the user problem, scope, verification,
privacy or compatibility impact, and recovery plan. CI must pass before merge.
Maintainers may close proposals that duplicate mature tools without a narrower,
evidence-backed use case.

By participating, you agree to follow the [Code of Conduct](CODE_OF_CONDUCT.md).
