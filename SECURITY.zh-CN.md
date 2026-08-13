# 安全政策

[English](SECURITY.md)

## 支持版本

项目尚未发布。第一版托管 release 之前，安全修复应用于 `main`。

| 版本 | 支持状态 |
| --- | --- |
| `main` | 支持 |
| 修改后的本地副本 | 不作承诺 |

## 报告漏洞

不要为漏洞创建公开 issue，也不要提交真实凭据或私有配置。请使用
[GitHub Private Vulnerability Reporting](https://github.com/Tinkora/json_yaml_swiss/security/advisories/new)，
并提供受影响 commit、影响、最小非敏感复现和建议的缓解方案。

仓库公开宣传前必须启用 Private Vulnerability Reporting。在该私密渠道可用前，请不要提交
敏感细节。

## 安全边界

浏览器应用通过本地 Rust/WASM 在内存中处理输入。它没有托管或远程转换 API、遥测、
持久化、service worker 或用户账号。用户可控值只通过表单 value 或 DOM 文本 API 渲染。
Content Security Policy 限制脚本、连接、表单和嵌入对象。

资源上限用于降低意外或恶意内存耗尽风险：输入 2 MiB、规范化节点 100,000 个、嵌套深度
128、输出 4 MiB。这些限制不代表浏览器 sandbox escape 不可能发生；此类报告仍在范围内。

新增网络访问、持久化、可执行配置、parser plugin 或 package 发布前，必须单独进行安全审查。
