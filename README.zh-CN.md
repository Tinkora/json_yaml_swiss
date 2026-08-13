# JSON YAML Swiss

在浏览器中检查、校验、格式化和明确转换 JSON、YAML 与 TOML。解析由本地
Rust/WebAssembly 完成；配置文本不会上传、存储或发送给遥测服务。

[English](README.md)

[![在 Ko-fi 上支持 Tinkora](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/tinkora)

## 为什么需要它

配置格式转换本身并不一定无损。JSON、YAML 和 TOML 的值模型与源码特性不同，许多转换器
会静默丢弃注释、覆盖重复键或强制转换不支持的值。JSON YAML Swiss 明确展示这条边界：
报告规范化行为、拒绝目标格式无法表示的值，也不会仅凭格式检测结果改变用户选择的来源格式。

当配置文件包含敏感信息、无法使用本地 CLI，或者需要在把文档交给脚本或 AI agent 前先
检查其结构时，可以使用本工具。

## 它能做什么

- 检查显式选择的 JSON、YAML 或 TOML 文档，报告根类型、节点数、深度和字节数。
- 列出所有能接受当前输入的 parser，同时保留格式歧义。
- 规范化单一格式，或执行全部六个跨格式转换方向。
- 拒绝重复键、不支持的 YAML 结构、非有限数值和目标格式无法表示的值。
- 用稳定 warning code 展示表现形式、注释、键顺序和 TOML datetime 的规范化。
- 通过英文默认、可切换中文的界面复制或下载结果。

## 转换契约

共享值模型支持 null、布尔值、有符号与无符号 64 位整数、有限浮点数、字符串、数组，以及
具有唯一字符串键的对象。

本工具不保留注释、空白、引号、标量样式、anchor、alias 或精确键顺序。YAML tag、
非字符串 mapping key 和多文档流会被拒绝。YAML merge key 会作为普通的 `<<` 数据保留，
绝不执行 merge 求值。TOML datetime 转为 JSON 或 YAML 时会变为字符串并返回 warning。
输出 TOML 时会拒绝 null 和非 object 根节点。

限制为：输入 2 MiB、规范化节点 100,000 个、嵌套深度 128、输出 4 MiB。完整契约和稳定
诊断码见[产品规格](docs/product_spec.zh-CN.md)。

## 浏览器工具

公开应用位于 <https://tinkora.github.io/json_yaml_swiss/>。

本地运行：

```console
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --version 0.15.0 --locked
cd crates/json_yaml_swiss_web
npm ci --ignore-scripts
npm run build:wasm
python3 -m http.server 8080
```

打开 <http://127.0.0.1:8080/static/>。

## 开发检查

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

浏览器测试会构建真实 release WASM，并在 375、768、1024、1440 像素的 Chromium 中
验证本地文件、稳定错误、恶意文本渲染、键盘操作、本地化、剪贴板、下载、console error 和
意外外部请求。

## 项目状态

项目当前处于 **Alpha**。native、WASM、浏览器、文档、依赖和安全检查均已在托管 CI 中
通过，部署后的 Pages 应用也已完成验证。参见
[成熟度](docs/MATURITY.zh-CN.md)、[贡献指南](CONTRIBUTING.zh-CN.md)、
[安全政策](SECURITY.zh-CN.md)、[支持说明](SUPPORT.zh-CN.md)和
[发布检查清单](docs/RELEASE_CHECKLIST.zh-CN.md)。

## 许可证与支持

项目使用 [MIT License](LICENSE) 发布。

如果本工具节省了你的时间，可以在 Ko-fi 上支持 Tinkora。赞助完全自愿，不是使用、获取支持或参与贡献的条件。
