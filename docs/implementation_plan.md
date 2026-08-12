# JSON YAML Swiss Implementation Plan

## Phase 1: Core contract

- [ ] Replace the legacy string-only conversion API with versioned inspection,
      detection, and conversion reports.
- [ ] Adopt the maintained `serde_yaml_ng` parser and keep parser versions locked.
- [ ] Add outcome-focused fixtures for format ambiguity, unsupported YAML values,
      TOML datetime normalization, null-to-TOML rejection, and resource limits.
- [ ] Verify native stable/MSRV tests, formatting, Clippy, and WASM compilation.

## Phase 2: Browser workflow

- [ ] Run `ui-ux-pro-max` and persist the selected design system.
- [ ] Build a single local-first inspector/converter surface backed only by the
      real WASM artifact.
- [ ] Render content as text, make warnings visible before export, and avoid all
      persistence and runtime requests.
- [ ] Add Chromium checks at 375, 768, 1024, and 1440 pixels.

## Phase 3: Public delivery

- [ ] Replace legacy claims with complete English-first and Chinese documentation.
- [ ] Add community health files, maturity/release docs, pinned reusable workflows,
      CodeQL, Dependabot, supply-chain checks, and Pages deployment.
- [ ] Remove draft MCP/Agent schema files because no runnable transport exists.
- [ ] Run the complete local gate and review the public diff.

## Phase 4: GitHub publication

- [ ] Create `Tinkora/json_yaml_swiss` as a public repository with the approved
      topics, discussions, security features, merge policy, and rulesets.
- [ ] Push `main`, enable Pages, and require the exact hosted checks.
- [ ] Verify the deployed application and hosted evidence for the exact commit.
- [ ] Keep maturity at Draft until release artifacts and release evidence exist.
