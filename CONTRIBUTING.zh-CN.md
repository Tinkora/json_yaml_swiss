# 贡献指南

感谢你改进 JSON YAML Swiss。贡献应对应一个可复现的配置处理问题，并优先选择不会削弱转换
契约的最小有效改动。

[English](CONTRIBUTING.md)

## 开始修改前

- 搜索现有 issue 和 discussion。
- 提议功能时，先说明用户痛点和现有替代方案，再开始实现。
- 不要在 issue、fixture、日志或截图中包含 secret 或私有配置。
- 安全问题请使用 GitHub Private Vulnerability Reporting。

## 开发环境

安装 Rust 1.95 或更新版本、`wasm32-unknown-unknown` target、Node.js 24、Python 3 和
`wasm-pack 0.15.0`。

```console
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --version 0.15.0 --locked
cd crates/json_yaml_swiss_web
npm ci --ignore-scripts
npx --no-install playwright install chromium
cd ../..
```

请求审查前运行完整本地门禁：

```console
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo check --workspace --target wasm32-unknown-unknown --locked
ruby scripts/test_check_docs.rb
ruby scripts/check_docs.rb
cargo deny check advisories bans licenses sources
cargo audit --deny warnings --no-yanked
npm --prefix crates/json_yaml_swiss_web audit --audit-level=high
npm --prefix crates/json_yaml_swiss_web run test:browser
```

## 修改规则

- Rust core 是权威实现；浏览器必须调用真实 WASM export，不得维护第二套 parser。
- 对不支持的值明确失败，不得静默强制转换或覆盖数据。
- 行为变化必须更新结果导向的测试，包括无效输入和受影响边界。
- 公开文档默认英文；用户可见含义变化时同步更新中文对等文件。
- 代码注释和 commit message 使用英文。commit 使用 `feat:`、`fix:`、`docs:`、`test:`、
  `chore:` 等 Conventional Commit 前缀。
- 修改 HTML、CSS 或浏览器用户界面前，必须运行 `ui-ux-pro-max` 技能，并验证 375、768、
  1024、1440 像素视口。

## Pull request

每个 pull request 保持聚焦，说明用户问题、范围、验证、隐私或兼容性影响和恢复方案。
CI 通过后才能合并。对于已有成熟工具覆盖、又没有更窄且有证据支持的使用场景，维护者可能
关闭对应提案。

参与项目即表示你同意遵守[行为准则](CODE_OF_CONDUCT.zh-CN.md)。
