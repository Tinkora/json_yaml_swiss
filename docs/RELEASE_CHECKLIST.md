# Release checklist

[简体中文](RELEASE_CHECKLIST.zh-CN.md)

This checklist validates a candidate; it does not by itself authorize
publication.

- [ ] The exact clean commit, SemVer version, and immutable `v<version>` tag are recorded.
- [ ] Rust formatting, workspace tests, Clippy, locked WASM checks, documentation checks, and supply-chain checks pass.
- [ ] `npm ci --ignore-scripts`, `npm audit --audit-level=high`, the release WASM build, and all four Chromium viewports pass.
- [ ] English and Chinese docs agree on limits, privacy, unsupported values, and maturity.
- [ ] GitHub hosted checks pass on the exact candidate commit; local success is not a substitute.
- [ ] The deployed Pages application loads the expected commit with no console error, external runtime request, overflow, or inaccessible primary workflow.
- [ ] Private Vulnerability Reporting, Discussions, security features, merge policy, and the main-branch ruleset are enabled.
- [ ] Release assets have deterministic names, SHA-256 checksums, an SPDX SBOM, and license evidence.
- [ ] The published GitHub Release contains exactly the verified candidate assets.
- [ ] The rollback or fix-forward owner and previous known-good artifact are recorded.
