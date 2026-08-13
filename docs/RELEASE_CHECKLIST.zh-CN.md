# 发布前检查清单

[English](RELEASE_CHECKLIST.md)

本清单用于验证候选版本，不单独代表获准发布。

- [ ] 已记录准确且干净的 commit、SemVer 版本和不可变 `v<version>` tag。
- [ ] Rust 格式、workspace 测试、Clippy、锁定的 WASM 检查、文档检查和供应链检查均通过。
- [ ] `npm ci --ignore-scripts`、`npm audit --audit-level=high`、release WASM 构建和四个 Chromium 视口均通过。
- [ ] 中英文文档对限制、隐私、不支持的值和成熟度描述一致。
- [ ] GitHub 托管检查在准确候选 commit 上通过；本地成功不能替代它。
- [ ] 已部署 Pages 应用对应预期 commit，且没有 console error、意外运行时外部请求、溢出或无法访问的主要流程。
- [ ] 已启用 Private Vulnerability Reporting、Discussions、安全功能、merge policy 和 main 分支 ruleset。
- [ ] 发布产物具有确定性名称、SHA-256 checksum、SPDX SBOM、许可证证据和 provenance。
- [ ] 已记录 rollback 或 fix-forward 负责人和上一份可用产物。
