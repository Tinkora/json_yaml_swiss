# Security Policy

[简体中文](SECURITY.zh-CN.md)

## Supported versions

| Version | Supported |
| --- | --- |
| `0.1.1` | Yes |
| `0.1.0` | No |
| `< 0.1` | No |
| Modified local copies | No commitment |

## Reporting a vulnerability

Do not open a public issue for a vulnerability or include real credentials or
private configuration. Use
[GitHub Private Vulnerability Reporting](https://github.com/Tinkora/json_yaml_swiss/security/advisories/new)
and provide the affected commit, impact, minimal non-sensitive reproduction,
and suggested mitigation.

Private Vulnerability Reporting is enabled for this repository.

## Security boundary

The browser application processes input in memory with local Rust/WASM. It has
no hosted or remote conversion API, telemetry, persistence, service worker, or
user account.
User-controlled values are rendered through form values or DOM text APIs. A
Content Security Policy limits scripts, connections, forms, and embedded
objects.

Resource limits reduce accidental or adversarial memory exhaustion: 2 MiB
input, 100,000 normalized nodes, 128 nesting levels, and 4 MiB output. These
limits do not make a browser sandbox escape impossible; such reports remain in
scope.

New network access, persistence, executable configuration, parser plugins, or
package publication requires a separate security review.
