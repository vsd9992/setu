# M1 read-only foundation — closed by project termination

## Outcome

A small Rust prototype was built and verified before the upstream overlap was discovered:

- Loopback-only Joplin URL validation.
- Content-free service and authentication diagnostics.
- Redacted environment-token handling and sanitized errors.
- Bounded, exact notebook allowlist resolution.
- Ten passing tests and clean formatting, compilation, and Clippy checks.
- Live validation against Joplin Desktop 3.6.15 and the `_setuDev` synthetic notebook.

## Closure

Joplin 3.6 native MCP satisfies the original product goal. The user chose to terminate Setu rather than reinvent and maintain overlapping functionality. Search/read tools, MCP transport, writes, packaging, and release work were not implemented.
