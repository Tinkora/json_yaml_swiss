# JSON YAML Swiss 产品规格

[English](product_spec.md)

状态：Alpha。最后复核：2026-08-13。

## 目标

JSON YAML Swiss 是一个本地浏览器工具，用于检查、校验、格式化和显式转换
JSON、YAML 与 TOML 配置数据。它服务于不希望把敏感配置粘贴到在线转换网站的开发者。

本工具不是无损源码编辑器。三种格式的数据模型不同，解析和重新序列化可能移除注释、
anchor、alias、标量样式、空白和键顺序。每次操作必须保留受支持的语义值、报告具体的
规范化行为，或者明确失败；绝不静默虚构键、强制转换不支持的值或覆盖重复映射项。

## MVP 工作流

1. 粘贴或加载 UTF-8 JSON、YAML 或 TOML 文档。
2. 显式选择源格式，或者请求一个非权威的格式建议；建议会列出所有能接受输入的 parser。
3. 校验语法并查看格式、根类型、节点数、最大深度、字节数和转换诊断。
4. 将 JSON、YAML 或 TOML 格式化为确定性的规范化输出。
5. 仅当目标格式可以表示规范化值时，执行六个方向的格式转换。
6. 阅读 warning 后复制或下载输出。

## 数据契约

### 限制

- 输入必须为 UTF-8，且不超过 2 MiB。
- 规范化文档最多包含 100,000 个值。
- 最大嵌套深度为 128。
- 生成结果不得超过 4 MiB。
- YAML 输入必须且只能包含一个文档。

### 支持的语义值

共享转换模型支持 null、布尔值、有符号和无符号 64 位整数、有限浮点数、字符串、数组，
以及具有唯一字符串键的对象。

边界如下：

- YAML tag 和非字符串 mapping key 在格式化与转换时都会被拒绝。
- 所选 parser 必须拒绝重复键；不得使用 first-wins 或 last-wins 行为处理。
- TOML datetime 在转换为其他格式时会变为字符串，并返回
  `TOML_DATETIME_STRINGIFIED`。
- JSON 或 YAML null 无法由 TOML 表示，必须返回
  `TARGET_CANNOT_REPRESENT_VALUE`。
- TOML 输出要求文档根节点为对象。
- 非有限浮点数会被拒绝。

### 规范化 warning

成功的格式化和转换返回一个带版本号的 report，包含输出和有序 warning code。warning
只描述实际转换，不声称源码保真：

- `COMMENTS_NOT_PRESERVED`
- `PRESENTATION_NOT_PRESERVED`
- `KEY_ORDER_NORMALIZED`
- `TOML_DATETIME_STRINGIFIED`

YAML 和 TOML 重新序列化会报告注释与表现形式变化。所有对象或 table 的键顺序都按确定性
规则规范化。

### 格式建议

格式检测只提供建议。结果包含所有通过 parser 的格式、可选建议和 `ambiguous` 标记。
JSON 有效时优先建议 JSON，因为 JSON 同时也是有效 YAML。只有 TOML parser 接受输入且
文本包含 TOML 特有的 key、赋值或 table 标记时才建议 TOML。界面中的源格式仍由用户
显式选择。

## 隐私与安全

- 解析、校验、格式化和转换均在本地 Rust/WASM 中完成。
- 静态资源加载后，应用不发起运行时网络请求。
- 用户输入不会写入 cookie、IndexedDB、`localStorage` 或 service worker cache。
- 用户可控文本只通过表单 value 或 `textContent` 渲染，绝不使用 `innerHTML`。
- 只有用户明确操作时才读取剪贴板或下载文件。
- 不允许 JavaScript parser fallback。

## 公开接口

Rust core 是唯一权威实现。WASM adapter 返回相同 report 和稳定错误的序列化形式。MVP
不声称提供 MCP server、Agent Skill、远程 API 或托管转换服务。

稳定错误码：

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

## 非目标

- 保留注释、anchor、alias、精确键顺序、空白、引号或标量样式。
- Schema 校验、JSON Schema 修复、merge key 求值或模板展开。
- YAML 多文档流。
- HCL、XML、INI、CSV、环境变量文件或任意插件格式。
- 执行配置、解析文件引用或访问远程 URL。
- 对需要 warning 或拒绝的值声称语义等价。

## 验证

- Parser 和转换 fixture 覆盖六个跨格式方向、格式化、歧义检测、重复键、YAML tag/非字符串键、
  TOML datetime、null 转 TOML 拒绝、资源上限和稳定诊断。
- clean checkout 通过 Rust 格式、测试、Clippy、MSRV 和 WASM 检查。
- 真实 Chromium 测试覆盖本地文件、校验、warning、转换、复制/下载状态、恶意文本渲染，
  以及 375、768、1024、1440 像素视口。
- 文档检查检查必需的双语文件、本地链接、UTF-8 编码和旧公开脚手架清理。
- Alpha 发布前，精确 public commit 的 Quality、Documentation quality、Supply chain、CodeQL
  和 Pages 必须全部通过。
