# Setu Project Plan

modified: 2026-08-13

Status: Superseded; project terminated 2026-08-13 after discovery of Joplin 3.6 native MCP.

## Objective

Build a small, local-first gateway that lets explicitly configured tools search, read, and safely modify a user's Joplin notes through Joplin Desktop's Data API, without copying the vault into another datastore or depending on an AI provider SDK.

## Why

Joplin exposes a capable local API, but direct use by AI clients creates unnecessary coupling and a broad security surface. Setu can provide stable, narrowly scoped operations, consistent errors, safer mutation semantics, and client-facing adapters.

## Evaluation of the initial proposal

The core idea is feasible and appropriately narrow. The following parts should remain:

- Joplin-only, local-first, provider-neutral scope.
- Joplin Desktop Data API/Web Clipper service as the first backend.
- No cloud service, LLM inference, vector database, telemetry, or destructive note operations in MVP.
- A layered core so transports do not duplicate Joplin logic.
- Secret-free public repository and redacted logs.

The following claims are premature or should change:

- A localhost HTTP API is not automatically callable by ChatGPT, Claude, or Gemini. Compatibility must be claimed per tested client and adapter, not by brand name.
- An API key does not provide granular authorization. The MVP should default to read-only and require explicit enablement of bounded write capabilities.
- Full note replacement and append can lose concurrent edits unless mutation requests carry a version precondition and conflicts fail closed.
- A public HTTP interface would add setup, privacy, and attack surface before proving value. The first AI adapter is local stdio MCP. ChatGPT developer-mode testing uses OpenAI's Secure MCP Tunnel; Setu will not expose a public endpoint in MVP.
- Notebook creation and tag mutation are useful but not required to prove the core note workflow; they should follow the first vertical slice.
- Rust is reasonable for distribution and safety, but it increases initial delivery cost. It should be accepted as a product constraint, not assumed to be validated by the repository.

## MVP scope

In scope:

- Configuration and diagnostic command.
- Joplin reachability and authentication check.
- Search notes and read one note with explicit field selection and pagination handling.
- Create a note and append/update with optimistic concurrency.
- Read-only default; separately enabled write capability.
- A local stdio MCP adapter, tested first with ChatGPT through Secure MCP Tunnel. Claude and Gemini follow as separately verified phases.
- Redacted structured diagnostics and stable domain errors.
- Automated tests against a mock Joplin server plus a documented manual test against disposable Joplin content.

Phased after the core workflow:

- Notebook creation and tag mutation.
- Claude and Gemini adapters/connections, reusing the core where their supported transports allow it.

Deferred until separately justified and security-reviewed:

- Attachments/resources, delete/trash, semantic search, GUI, installers, cloud hosting/public proxy, multi-user operation, provider SDKs, and generic automation.

## Boundaries and assumptions

- Setu talks to a locally running Joplin Desktop API, never directly to a sync backend or Joplin database.
- Setu stores no note index or note bodies persistently in MVP.
- Setu does not accept remote/LAN connections in MVP. ChatGPT testing may use Secure MCP Tunnel as an explicit outbound bridge to the local stdio MCP process; a public endpoint or proxy is a separate future design requiring approval.
- Tests use synthetic fixtures. Real-Joplin verification uses disposable test notes and must not expose user content.
- Client support is documented only after an end-to-end test on a named client/version and platform.
- Local-first does not mean local-only when a cloud AI client is used: note data returned to that client leaves the device. Setup and each enabled integration must disclose this boundary clearly.

## Success criteria

- A new user can configure Setu without placing secrets in the repository or command history documented by the project.
- Search/read work end-to-end against supported Joplin Desktop versions.
- Writes are disabled by default and reject stale versions without overwriting newer content.
- Tokens and note bodies do not appear in normal logs or error responses.
- ChatGPT can complete the verified search-read-create/append workflow through the approved tunnel path without a public Setu endpoint.
- Users can see and control which read/write capabilities and data scopes are enabled.
- No milestone ships until its security and privacy verification criteria pass.
- Supported operating systems, clients, and limitations are stated from evidence.

## High-level phases

1. Validate uncertain external behaviours and threat boundaries.
2. Build the smallest diagnostic and read-only vertical slice.
3. Add guarded mutations and conflict handling.
4. Verify ChatGPT integration through Secure MCP Tunnel.
5. Add notebook/tag capabilities, then evaluate Claude and Gemini separately.
6. Harden, document, package, and release a narrow preview.
