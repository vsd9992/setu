# Use Rust for the implementation

## Decision

Implement Setu in Rust from the first executable milestone.

## Basis

The user approved the proposed language direction. Rust supports a typed, auditable core and cross-platform binary distribution.

## Why

The long-term safety and distribution benefits justify the initial development cost for this security-sensitive local tool.

## Important consequences/constraints

- Keep dependencies narrow and review licenses and advisories.
- Validate MCP SDK/protocol options before selecting crates; do not weaken boundaries merely to use a convenient framework.
