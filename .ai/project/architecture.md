# Setu Architecture

modified: 2026-08-13

Status: Superseded; no production architecture will be implemented.

## Components

```text
ChatGPT -> Secure MCP Tunnel -> local stdio MCP adapter
                         |
                 capability/auth boundary
                         |
                  application services
                         |
               Joplin adapter + redaction
                         |
          Joplin Desktop Data API on loopback
```

- Client adapter: validates bounded inputs, authenticates where transport requires it, and maps stable responses. It contains no Joplin-specific business logic.
- Capability boundary: enforces read-only default and explicit write permissions before any Joplin call.
- Application services: implement search, read, create, and guarded mutation semantics using Setu domain types.
- Joplin adapter: owns API authentication, field selection, pagination, timeouts, error conversion, and protection against token leakage.
- Configuration/diagnostics: resolve configuration once, expose sanitized status, and never return secret values.

## Invariants

- No adapter can bypass capability checks.
- No raw Joplin API pass-through exists.
- A mutation fails if its expected version no longer matches Joplin state.
- Joplin tokens never cross the adapter boundary or enter logs/errors.
- Note data is processed in memory and is not persisted by Setu.
- Setu exposes no public or LAN listener in MVP.
- Every tool returns the minimum data required for its declared purpose.
- Cloud-client disclosure is part of setup and cannot be represented as fully local processing.

## Transport decision

- Local stdio MCP is the first adapter and keeps Setu from opening a listening port.
- ChatGPT developer mode requires a public HTTPS MCP endpoint or Secure MCP Tunnel. Setu uses Secure MCP Tunnel for development/testing so the local-first MVP does not require a public endpoint.
- Public ChatGPT plugin submission requires a stable public HTTPS endpoint and is outside the MVP architecture.
- Claude and Gemini are later compatibility phases; their current supported connection mechanisms must be verified before adapter work begins.

The application core remains transport-independent so later verified adapters do not duplicate Joplin or authorization logic.

## Data and state

MVP state is configuration only. Setu has no database, cache, index, synchronization engine, or background write queue. Joplin remains authoritative for notes and versions.
