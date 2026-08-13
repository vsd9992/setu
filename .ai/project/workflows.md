# Setu Workflows

modified: 2026-08-13

Status: Superseded; retained for historical reference only.

## Setup and diagnosis

1. User enables Joplin's local API and obtains its token.
2. User stores the token in an environment variable for the developer preview and selects Setu capabilities; default is read-only.
3. User may configure a notebook allowlist. When present, every data operation is constrained to those notebooks.
4. Diagnostic command validates configuration, loopback endpoint, reachability, and authentication.
5. Output reports sanitized pass/fail guidance without note access or secret values.

## Connect ChatGPT

1. User reviews the disclosure that note data returned to ChatGPT leaves the device and is governed by the user's OpenAI account/workspace controls.
2. User starts Setu's local stdio MCP adapter in read-only mode.
3. User explicitly configures OpenAI Secure MCP Tunnel for that local process and enables ChatGPT developer mode where available.
4. User reviews discovered tools and verifies that only enabled capabilities are advertised.
5. The user controls what is sent by selecting allowed notebooks and by intentionally invoking full-note reads; search returns bounded excerpts by default.
6. The evaluation set tests intended reads, unsupported requests, authorization failures, write confirmations, bounded results, and absence of credential leakage.
7. The tunnel is stopped or disconnected when the integration is not wanted.

## Search and read

1. Client invokes a bounded search through the approved adapter; setup and tool descriptions disclose the cloud data boundary.
2. Setu validates query and limits, checks read capability, and calls Joplin with explicit fields.
3. Setu enforces any notebook allowlist, follows pagination only within configured bounds, and returns at most 10 summaries with excerpts of at most 300 characters by default.
4. Client selects an identifier and reads the note.
5. Setu rejects notes outside the allowlist. If no allowlist is configured, a full-note read requires user confirmation.
6. Setu returns content plus a version value suitable for a later guarded mutation.

## Create

1. Client submits bounded title/body/notebook inputs.
2. Setu rejects the request unless note creation is enabled, the destination notebook is allowed, and the client/user confirmation boundary is satisfied.
3. Setu validates fields and creates the note through the adapter.
4. Setu returns identifier, selected metadata, and version; it does not log the body.

## Guarded append or update

1. Client first reads the note and retains its version.
2. Client submits note identifier, expected version, and bounded content.
3. Setu checks the notebook scope, write capability, and either per-action confirmation or a valid session-trust grant, then revalidates current Joplin state.
4. If versions differ, Setu returns a conflict and makes no write.
5. Otherwise Setu applies the mutation and returns the new version.

Append is a convenience operation, not inherently concurrency-safe; it follows the same precondition flow as update.

## Temporary session trust

1. Confirmation is required for every write by default.
2. The user may explicitly trust selected write capabilities for selected allowed notebooks for the current Setu process session.
3. Setu displays the active trust scope and applies it only to matching operations.
4. The grant is held only in memory and ends on explicit revocation or process exit.
5. Session trust never bypasses notebook scope, input bounds, authorization, or optimistic concurrency.

## Compatibility verification

1. Use the dedicated test notebook in the user's local Windows Joplin installation and populate it only with synthetic content.
2. Record OS, Joplin version, Setu version/commit, adapter, and client/version.
3. Run diagnosis, search, read, create, append/update, stale-write rejection, data-minimization, confirmation, and secret-redaction checks.
4. Claim support only for combinations that pass the documented checks.
