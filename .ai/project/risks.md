# Active Project Risks

modified: 2026-08-13

## High: overbroad AI-client compatibility claims

Different desktop and hosted clients have different support for local HTTP, MCP transports, authentication, and user confirmation. Mitigation: select one primary adapter and publish only tested compatibility claims.

## High: unintended or stale note writes

AI callers may target the wrong note or overwrite concurrent edits. Mitigation: read-only default, explicit write capabilities, bounded inputs, expected-version preconditions, conflict errors, and disposable end-to-end tests.

## High: credential or note leakage

Joplin tokens may appear in URLs; request tracing and errors can expose them or note bodies. Mitigation: central redaction, no body logging, sanitized diagnostics, synthetic fixtures, and dedicated leakage tests.

## High: local-first is mistaken for local-only privacy

When ChatGPT or another cloud client reads a note, the returned content leaves the device. Mitigation: prominent disclosure, explicit connection setup, minimal tool results, bounded search excerpts, intentional full-note reads, and provider-specific privacy guidance without making guarantees Setu cannot enforce.

## High: tunnel or future hosted bridge expands the trust boundary

Secure MCP Tunnel connects a cloud client to the local process; a future public plugin would require an always-reachable HTTPS endpoint. Mitigation: tunnel only for explicitly configured development/testing, no public endpoint in MVP, narrow authorization and tool schemas, and a new approved threat model before hosting/proxy work.

## Medium: loopback is treated as a complete trust boundary

Other local processes and browser-origin attacks may reach a loopback server. Mitigation: prefer no listener when practical; otherwise require authentication plus host/origin defenses and strict bounds. Reject remote binding in MVP.

## Medium: Joplin API semantics are assumed rather than verified

Pagination, fields, mutation/version behaviour, and compatibility may differ across releases. Mitigation: validate against official documentation and representative stable Joplin versions before freezing contracts.

Official documentation currently exposes `updated_time` but no atomic conditional-update operation. A pre-write recheck followed by `PUT` retains a race window for external Joplin edits. Guarded writes remain blocked pending synthetic testing and an explicit resolution that preserves or revises the approved guarantee.

## Medium: premature architecture and documentation breadth

Multiple transports and extensive repository scaffolding can delay proof of value. Mitigation: one vertical slice, one primary adapter, compact baseline, and just-in-time public documentation.

## High: native Joplin MCP overlaps Setu's original proposition

Joplin 3.6 includes a built-in HTTP MCP server with search, read, and write tools. A generic Joplin-to-MCP bridge would now duplicate maintained upstream functionality. Mitigation: require explicit confirmation that Setu is a stricter security/privacy policy gateway, and retain only capabilities that measurably improve notebook scoping, disclosure bounds, confirmation, secret hygiene, or mutation safety over native Joplin MCP.

## Medium: cross-platform distribution cost

Rust binaries are viable, but signing, antivirus reputation, credential storage, and installers are separate concerns. Mitigation: start with developer builds on one approved platform and expand only after workflow validation.
