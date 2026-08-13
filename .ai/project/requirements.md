# Setu Requirements

modified: 2026-08-13

Status: Superseded; retained as historical evaluation input after project termination on 2026-08-13.

## Functional

- FR-01: Load non-secret settings from documented defaults, a local ignored config file, environment variables, and explicit CLI flags using a documented precedence order.
- FR-02: Load secrets only from environment variables, interactive input, or a later approved OS credential store; never echo them.
- FR-03: Diagnose configuration, Joplin reachability, and Joplin authentication without reading or logging user notes.
- FR-04: Search notes with bounded results, explicit pagination, and a stable Setu response model.
- FR-05: Read a note by identifier and request only fields needed by the operation.
- FR-06: Create a note only when the write capability is explicitly enabled.
- FR-07: Update or append only when the caller supplies the version observed during a prior read; reject stale mutations as conflicts.
- FR-08: Expose capabilities so a client can determine whether Setu is read-only or permits specific writes.
- FR-09: Provide local stdio MCP as the primary adapter, test it first with ChatGPT through Secure MCP Tunnel, and embed no AI-provider SDK in the core.
- FR-10: Normalize Joplin connection, authentication, validation, not-found, conflict, and internal failures into stable errors without secret leakage.
- FR-11: Support an optional notebook allowlist. When configured, search, read, create, and mutation operations must remain within allowed notebooks.
- FR-12: Default to confirmation for each write and allow the user to grant temporary session trust for selected write capabilities and notebooks.

## Security and privacy

- SR-01: Bind network listeners to loopback by default; MVP must reject non-loopback binding rather than merely warn.
- SR-02: Default to read-only. Each write category requires explicit configuration.
- SR-03: Authenticate every network request except a minimal liveness response; liveness must disclose no configuration or dependency details.
- SR-04: Use constant-time comparison for static bearer credentials if an HTTP adapter is approved.
- SR-05: Never log credentials, authorization headers, Joplin request URLs containing tokens, or full note bodies by default.
- SR-06: Place explicit request-size, response-size/result-count, and timeout bounds on external interfaces.
- SR-07: Do not persist note content, search indexes, or operational request logs in MVP.
- SR-08: Do not support delete, trash, raw pass-through API calls, or arbitrary Joplin fields in MVP.
- SR-09: Minimize data returned to AI clients through explicit fields, bounded excerpts/results, and narrow tool schemas; full note bodies require an intentional read operation.
- SR-10: Clearly disclose that data returned to a cloud AI client leaves the local device and is subject to that provider/account's data controls.
- SR-11: Require an explicit user confirmation boundary for consequential write operations in addition to server-side authorization.
- SR-12: Treat security and privacy verification failures as release blockers for every milestone.
- SR-13: Do not create a public Setu endpoint, hosted relay, or public proxy in MVP. Such a change requires a new threat model and explicit approval.
- SR-14: A session-trust grant must be explicit, non-persistent, capability-scoped, notebook-scoped when an allowlist is active, visible to the user, and revoked when the Setu session ends. It cannot bypass authorization, validation, bounds, or version checks.
- SR-15: When no notebook allowlist is configured, a full-note read requires confirmation. Within an explicit allowlist, full-note reads may proceed without per-read confirmation.
- SR-16: Search defaults to at most 10 results and 300 characters of matching excerpt per result. Broader values require explicit bounded configuration.
- SR-17: All real-Joplin development tests must use a dedicated test notebook containing only synthetic test content; personal notes are out of scope for automated or exploratory tests.

## Quality and compatibility

- QR-01: Core domain behaviour and Joplin mapping must be transport-independent.
- QR-02: Automated tests cover config precedence, secret redaction, auth, bounds, pagination, error mapping, and stale-write rejection.
- QR-03: Integration tests must not require a user's live Joplin profile.
- QR-04: Real-Joplin tests use disposable synthetic notes and record Joplin version/platform.
- QR-05: Client or platform compatibility claims require a recorded successful end-to-end check.
- QR-06: The release process must generate reproducible provenance/checksum information before distributing binaries.
- QR-07: Security-sensitive code must receive negative-path tests and dependency/advisory review before release.
- QR-08: The first supported development-preview platform is Windows. Portability must be preserved, but other platforms are not claimed until tested.
- QR-09: Developer-preview secrets use environment variables. Windows Credential Manager support is required before claims of general Windows usability.

## External behaviour still to verify

- EV-01: Exact Joplin version token and mutation sequence for optimistic concurrency.
- EV-02: Secure MCP Tunnel lifecycle, local process configuration, and availability for the test account/workspace.
- EV-03: Claude and Gemini connection mechanisms before their phases are planned.
- EV-04: Installed Joplin version and relevant Data API behavior on the user's Windows test machine.
