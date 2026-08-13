# Setu Roadmap

modified: 2026-08-13

## M0 — Planning and evidence

Status: Completed

Deliverables:

- Approved objective, scope, requirements, architecture, workflows, risks, and verification criteria.
- Confirmed stdio MCP/ChatGPT-first adapter, Rust, read-only default, guarded writes, and security/privacy release gate.
- Verified relevant Joplin API behaviour from documentation and a disposable local instance.

Dependencies: User's local Windows Joplin installation with the dedicated `_setuDev` synthetic test notebook (available); ChatGPT Developer mode and Secure MCP Tunnel availability (confirmed; no tunnel created yet).

Exit criteria: Installed Joplin version and relevant behavior are recorded; ChatGPT developer mode/tunnel availability is checked; privacy disclosures and security test gates are defined for M1.

Current evidence: Joplin Desktop 3.6.15 and Rust 1.93.1 are available; the loopback clipper service responds. ChatGPT Developer mode and Secure MCP Tunnel are available. Official Joplin documentation confirms fields and pagination but documents no atomic conditional update; synthetic mutation behavior remains unverified and blocks writes, not the read-only M1 slice.

## M1 — Diagnostic read-only vertical slice

Status: Cancelled after partial prototype validation

Deliverables: Rust toolchain foundation, environment-variable secret loading, sanitized diagnostics, notebook allowlist, Joplin adapter, bounded search/read, local stdio MCP adapter, and automated mock-server tests.

Dependencies: M0.

Verification criteria: Build/lint/tests pass; missing/invalid credentials are handled safely; allowlist bypass, token/body leakage, full-read confirmation, and data-minimization checks pass; search/read work against the synthetic test notebook; ChatGPT connects through Secure MCP Tunnel; Windows/Joplin/client versions are recorded. Any security/privacy failure blocks completion.

## M2 — Guarded note writes

Status: Cancelled

Deliverables: Explicit create and mutation capabilities, per-write confirmation, scoped in-memory session trust, create, guarded append/update, stable conflicts, and negative security tests.

Dependencies: M1 and verified Joplin mutation/version semantics.

Verification criteria: Read-only mode rejects all writes; notebook scope cannot be bypassed; confirmation and session-trust expiry behave correctly; allowed create/append/update work; stale mutations make no change; size limits and redaction tests pass.

## M3 — Organization capabilities

Status: Cancelled

Deliverables: Notebook creation and tag mutation behind separate capabilities.

Dependencies: M2.

Verification criteria: Read-only mode rejects organization writes; enabled operations are bounded and confirmed; regression and privacy checks pass.

## M4 — ChatGPT evaluation and preview hardening

Status: Cancelled

Deliverables: Windows Credential Manager integration, setup/security/API documentation, capability discovery, supported-client matrix, release checks, and Windows preview artifacts if approved.

Dependencies: M3; M3 capabilities may be omitted from the first preview if they are not ready.

Verification criteria: A clean-machine walkthrough succeeds from documentation; threat-focused tests pass; distributed artifacts have checksums/provenance; limitations are explicit.

## M5 — Additional AI clients

Status: Cancelled

Deliverables: Separately researched and tested Claude support, followed by Gemini support where feasible.

Dependencies: Stable core and client-specific security/privacy evaluation.

Verification criteria: Each named client/version passes the end-to-end evaluation set; unsupported or unsafe connection modes are documented rather than worked around insecurely.

## Later candidates

None. Project terminated on 2026-08-13 because Joplin 3.6 native MCP satisfies the original product goal. The prototype is preserved for traceability and is not supported or released.
