# Setu Project Plan

Setu is a public, local-first bridge between **Joplin** and AI assistants such as ChatGPT, Claude, Gemini, local LLM tools, and future MCP-capable clients.

This document is the primary planning document for the repository. It is intentionally detailed so a fresh development session, AI coding agent, or new contributor can understand the project direction without depending on private chat history.

Repository: <https://github.com/vsd9992/setu>

---

## 1. Project Summary

Setu lets an AI assistant interact with a user's local Joplin notes in a controlled way.

The first implementation will use the **Joplin Web Clipper REST API** because it is already available in stable Joplin desktop builds and works locally.

Setu should expose a safer, simpler, AI-friendly interface over Joplin rather than forcing AI clients to use Joplin's raw API directly.

### One-line description

> A local AI gateway for Joplin.

### Short public description

> Setu connects Joplin with AI assistants through a secure local gateway, allowing controlled note search, reading, creation, and updates.

### Long public description

> Setu is a lightweight open-source local gateway that allows AI assistants such as ChatGPT, Claude, Gemini, and local LLM tools to work with Joplin notes. It runs on the user's machine, talks to Joplin through the Web Clipper REST API, and exposes a stable, assistant-friendly interface for note search, reading, creation, updates, notebooks, and tags.

---

## 2. Product Positioning

Setu is not trying to replace Joplin. It is not trying to become an AI note app. It is a bridge.

The user keeps Joplin as the trusted knowledge base. Setu provides a controlled local interface so AI tools can work with that knowledge base.

### Target value

Setu should help users say things like:

- Search my Joplin notes for this topic.
- Read this note and summarize it.
- Create a note from this conversation.
- Append this decision to my project log.
- Add tags to this note.
- Find related notes.
- Create a clean note without exposing my full vault to a cloud note service.

### Key design promise

Setu should be:

- Local-first.
- AI-provider neutral.
- Joplin-focused.
- Secure by default.
- Simple to install.
- Simple to audit.
- Useful without cloud hosting.

---

## 3. Scope

### In scope

Setu will support Joplin operations useful for AI-assisted note management:

- Health check.
- Joplin connection check.
- Note search.
- Note read.
- Note create.
- Note update.
- Note append.
- Notebook list.
- Notebook create.
- Tag list.
- Tag add/remove.
- Safe configuration.
- Local authentication for Setu's own API.
- CLI helper commands.
- Documentation and examples for AI clients.
- Future MCP server mode.

### Out of scope

Setu will not support, at least in the core project:

- Non-Joplin note apps.
- Email.
- Calendar.
- GitHub automation.
- Docker management.
- ERPNext or business systems.
- General local file automation.
- Cloud-hosted Setu service.
- AI chat UI.
- LLM inference.
- Prompt management framework.
- Embeddings database.
- Vector search engine.
- Telemetry.
- User tracking.

Future integrations should be separate projects unless they directly serve Joplin.

---

## 4. Non-Goals

Setu is **not**:

- A replacement for Joplin.
- A Joplin sync target.
- A Joplin plugin at the start.
- A cloud note service.
- A general automation platform.
- A personal knowledge management methodology.
- A vendor-specific ChatGPT, Claude, or Gemini plugin.
- An AI model wrapper.
- A place to store secrets, tokens, exported notes, or private user content.

This matters because open-source tools often rot when they expand into every nearby idea. Setu should remain narrowly useful.

---

## 5. Critical Security Rule

This is a **public GitHub repository**.

No secrets or private user data may be stored in the repository.

### Never commit

- Joplin Web Clipper token.
- OpenAI API key.
- Anthropic API key.
- Google/Gemini API key.
- Any AI provider key.
- Setu local API key.
- `.env` files.
- `config.toml` with real values.
- Local logs containing note data.
- Joplin exported notes.
- Joplin profile data.
- SQLite databases from Joplin.
- Screenshots containing tokens or private notes.
- Private user notes copied for testing.
- Personal or business documents.

### Safe to commit

- `.env.example`.
- `config.example.toml`.
- Mock notes created specifically for tests.
- Fake tokens such as `change-me` or `example-token`.
- Documentation explaining how users configure their own local secrets.

### Secret handling policy

Secrets must be loaded from one of these sources:

1. Environment variables.
2. Local ignored config file.
3. Interactive CLI input.
4. OS keychain integration, if added later.

### If a secret is committed

Required response:

1. Revoke or rotate the secret immediately.
2. Remove it from active files.
3. Rewrite history if exposure risk justifies it.
4. Update `.gitignore`, docs, and tests to prevent recurrence.
5. Treat the leaked value as permanently compromised.

---

## 6. Target Users

### Primary users

- Joplin desktop users.
- Users who want AI assistance without moving notes into a cloud AI note app.
- Users comfortable enabling Joplin Web Clipper.
- Users running ChatGPT Desktop, Claude Desktop, Gemini tools, local LLM tools, or scripts.

### Secondary users

- Developers building AI workflows around Joplin.
- Power users who want automation but not cloud lock-in.
- Privacy-conscious users who prefer local-first tools.

### Assumed technical level

The project should be usable by non-developers eventually, but the first versions may require comfort with:

- Installing a binary.
- Running a local command.
- Setting environment variables.
- Reading setup instructions.
- Copying a Joplin Web Clipper token.

---

## 7. Operating Model

Initial architecture:

```text
AI Assistant / Client
        |
        v
Setu local API
        |
        v
Joplin Web Clipper REST API
        |
        v
Joplin Desktop
        |
        v
User's Joplin vault and sync target
```

Important point: Setu talks to the local Joplin Desktop application, not directly to OneDrive, Dropbox, Nextcloud, Joplin Cloud, or any sync backend.

### Local endpoints

Typical Joplin Web Clipper address:

```text
http://127.0.0.1:41184
```

Typical Setu address:

```text
http://127.0.0.1:9876
```

Both should be configurable. Setu must bind to localhost by default.

---

## 8. Compatibility Model

### Joplin compatibility

Initial assumption:

- Joplin Desktop stable release.
- Web Clipper enabled.
- Joplin Web Clipper token available to the user.
- Joplin running on the same machine as Setu.

The implementation must verify exact Web Clipper API behavior during development using the official Joplin documentation and real local testing.

### AI client compatibility

Setu should not depend on any one AI provider.

Supported or planned client styles:

1. Plain HTTP client.
2. CLI commands.
3. MCP server mode.
4. Copy-paste helper mode for clients that cannot call local services.
5. Future wrappers for specific clients only if they remain thin and optional.

### Important design rule

No OpenAI SDK, Anthropic SDK, or Gemini SDK should be required for Setu core.

---

## 9. Technology Direction

Preferred implementation language: **Rust**.

### Why Rust

- Good for small local tools.
- Strong binary distribution story.
- Cross-platform support.
- Good performance with low memory use.
- Strong type system.
- Safer error handling than quick scripts.
- Suitable for public open-source infrastructure.

### Likely dependencies

Initial likely stack:

```text
axum       - local HTTP server
tokio      - async runtime
reqwest    - Joplin REST client
serde      - JSON serialization
toml       - config parsing
clap       - CLI commands
tracing    - structured logging
thiserror  - library error types
anyhow     - application boundary errors
```

Dependency policy:

- Prefer the standard library where practical.
- Add dependencies only when they clearly reduce risk or complexity.
- Avoid large frameworks.
- Avoid dependencies that pull in unnecessary subsystems.
- Review licenses before adding dependencies.

---

## 10. Product Architecture

Setu should have clear internal layers.

```text
CLI / HTTP / MCP adapters
          |
          v
Setu domain services
          |
          v
Joplin adapter
          |
          v
Joplin Web Clipper REST API
```

### Layer responsibilities

#### CLI layer

- Parse user commands.
- Run `doctor` checks.
- Start server.
- Print human-friendly errors.

#### HTTP layer

- Expose local JSON API.
- Enforce Setu API authentication.
- Validate request bodies.
- Return stable response envelopes.

#### Future MCP layer

- Expose Setu operations as MCP tools.
- Reuse domain services.
- Avoid duplicating Joplin logic.

#### Domain layer

- Define Setu concepts: Note, Notebook, Tag, SearchResult.
- Apply Setu-specific rules.
- Hide Joplin API quirks from callers.

#### Joplin adapter

- Call Joplin Web Clipper API.
- Convert Joplin responses into Setu domain models.
- Handle pagination.
- Handle token use.
- Normalize errors.

---

## 11. Public API Philosophy

Setu should not mirror the full raw Joplin API.

Raw Joplin API access is powerful but too broad for AI clients. Setu should expose a smaller, safer, assistant-friendly API.

### API principles

- Stable names.
- Predictable request bodies.
- Predictable response envelopes.
- Clear error codes.
- No secrets in responses.
- No accidental destructive operations.
- Small surface area.
- Backward compatibility whenever practical.

### Initial HTTP endpoints

```text
GET  /health
POST /joplin/check
POST /note/search
POST /note/read
POST /note/create
POST /note/update
POST /note/append
POST /notebook/list
POST /notebook/create
POST /tag/list
POST /tag/add
POST /tag/remove
```

### Deferred endpoints

```text
POST /note/delete
POST /note/trash
POST /resource/upload
POST /resource/list
POST /note/link-related
```

Destructive operations should be delayed until the safety model is mature.

---

## 12. Response Format

Setu responses should use a consistent envelope.

### Success response

```json
{
  "ok": true,
  "data": {
    "id": "example-note-id",
    "title": "Example Note"
  }
}
```

### Error response

```json
{
  "ok": false,
  "error": {
    "code": "JOPLIN_CONNECTION_FAILED",
    "message": "Could not connect to Joplin Web Clipper at http://127.0.0.1:41184"
  }
}
```

### Error code examples

```text
CONFIG_NOT_FOUND
CONFIG_INVALID
SETU_AUTH_REQUIRED
SETU_AUTH_FAILED
JOPLIN_CONNECTION_FAILED
JOPLIN_AUTH_FAILED
JOPLIN_NOT_FOUND
JOPLIN_BAD_REQUEST
NOTE_NOT_FOUND
NOTE_UPDATE_CONFLICT
VALIDATION_FAILED
UNSUPPORTED_OPERATION
INTERNAL_ERROR
```

---

## 13. Configuration Model

Setu should support config through:

1. Environment variables.
2. Local config file.
3. CLI flags where useful.

Priority order should be documented during implementation.

### Example config file

Safe committed template: `config.example.toml`

```toml
[server]
host = "127.0.0.1"
port = 9876

[joplin]
base_url = "http://127.0.0.1:41184"
token_env = "JOPLIN_TOKEN"

[security]
api_key_env = "SETU_API_KEY"
```

### Local ignored files

The real local config must not be committed.

```text
.env
config.toml
config.local.toml
*.local.toml
```

### Environment variables

```text
JOPLIN_TOKEN
SETU_API_KEY
SETU_CONFIG
SETU_HOST
SETU_PORT
JOPLIN_BASE_URL
```

Exact names may be revised before v0.1, but secrets must remain outside Git.

---

## 14. Security Model

Setu is local-first but still security-sensitive because it can read and modify a user's notes.

### Default security posture

- Bind to `127.0.0.1` by default.
- Require Setu API key for all HTTP API calls except `/health`.
- Never expose the Joplin token.
- Never log secrets.
- Never log full note bodies by default.
- Warn loudly if binding to non-localhost.
- Do not implement delete/trash in MVP.

### Future permission profiles

Possible future profiles:

```text
read-only       - search and read only
write-notes     - create/update/append notes
tags            - tag editing allowed
notebooks       - notebook creation allowed
full            - all supported operations
```

### Destructive operations

Delete/trash should require:

- Explicit feature enablement.
- Separate permission.
- Clear request flag.
- Strong documentation.

Default should be no destructive operations.

---

## 15. Repository Structure

Target repository structure:

```text
setu/
├── README.md
├── PROJECT_PLAN.md
├── AGENTS.md
├── CONTRIBUTING.md
├── SECURITY.md
├── ROADMAP.md
├── CHANGELOG.md
├── LICENSE
├── .gitignore
├── .editorconfig
├── config.example.toml
├── Cargo.toml
├── rustfmt.toml
├── clippy.toml
│
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config.rs
│   ├── error.rs
│   ├── cli/
│   │   ├── mod.rs
│   │   ├── commands.rs
│   │   └── doctor.rs
│   ├── server/
│   │   ├── mod.rs
│   │   ├── routes.rs
│   │   ├── auth.rs
│   │   └── response.rs
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── note.rs
│   │   ├── notebook.rs
│   │   ├── tag.rs
│   │   └── search.rs
│   ├── joplin/
│   │   ├── mod.rs
│   │   ├── client.rs
│   │   ├── notes.rs
│   │   ├── notebooks.rs
│   │   ├── tags.rs
│   │   └── models.rs
│   └── mcp/
│       └── mod.rs        # placeholder until MCP is implemented
│
├── tests/
│   ├── health.rs
│   ├── notes.rs
│   └── fixtures/
│       └── mock_notes.json
│
├── examples/
│   ├── curl/
│   │   ├── search-note.sh
│   │   ├── create-note.sh
│   │   └── append-note.sh
│   └── prompts/
│       ├── chatgpt.md
│       ├── claude.md
│       └── gemini.md
│
├── docs/
│   ├── setup.md
│   ├── architecture.md
│   ├── api.md
│   ├── security.md
│   ├── development.md
│   ├── joplin-api-notes.md
│   ├── ai-client-examples.md
│   ├── release-process.md
│   └── decisions/
│       └── 0001-use-joplin-web-clipper-first.md
│
├── .ai/
│   ├── 00-project-index.md
│   ├── 01-product-brief.md
│   ├── 02-architecture-memory.md
│   ├── 03-decisions.md
│   ├── 04-task-board.md
│   ├── 05-risks.md
│   ├── 06-test-plan.md
│   ├── 07-debug-log.md
│   ├── 08-release-notes.md
│   └── 09-agent-handoff.md
│
└── .github/
    └── workflows/
        ├── ci.yml
        └── release.yml
```

---

## 16. Repository Memory Structure

The repository itself is the project memory.

The memory system is designed so a separate AI coding project can clone this repo, read the project files, and continue from the current state without needing private conversation history.

### Truth hierarchy

When sources conflict, use this order:

1. Code and tests.
2. Public documentation in `README.md` and `docs/`.
3. `PROJECT_PLAN.md`.
4. `.ai/` memory files.
5. Conversation history or external assumptions.

`.ai/` is useful context, not final authority.

### Memory goals

The `.ai/` directory should:

- Preserve decisions.
- Preserve current task state.
- Preserve known risks.
- Preserve debugging lessons.
- Help AI coding agents start quickly.
- Avoid duplicated documentation.
- Avoid becoming a transcript archive.

### Memory non-goals

The `.ai/` directory must not:

- Store private notes.
- Store tokens.
- Store logs with user content.
- Duplicate all docs.
- Replace tests.
- Replace source inspection.
- Become a large unstructured dump.

### Recommended `.ai/` files

#### `.ai/00-project-index.md`

Purpose: entry point for AI agents.

Should contain:

- Project summary.
- Current implementation status.
- Important files to read first.
- Current milestone.
- Links to key docs.
- Last updated date.

Should not contain:

- Full project plan.
- Private notes.
- Long historical commentary.

#### `.ai/01-product-brief.md`

Purpose: product intent and user value.

Should contain:

- Target users.
- Main use cases.
- Product constraints.
- Non-goals.
- UX principles.

#### `.ai/02-architecture-memory.md`

Purpose: concise architecture state.

Should contain:

- Current architecture summary.
- Major modules.
- Boundaries between layers.
- Interfaces that must remain stable.
- Known technical constraints.

Do not duplicate `docs/architecture.md`; summarize current working assumptions.

#### `.ai/03-decisions.md`

Purpose: decision ledger.

Each decision should use this format:

```md
## YYYY-MM-DD - Decision title

Status: accepted | rejected | superseded | proposed

Decision:
Short decision statement.

Reason:
Why this was chosen.

Consequences:
Tradeoffs, risks, or follow-up work.
```

Initial decisions to record:

- Use Joplin Web Clipper REST API first.
- Keep Setu Joplin-only.
- Keep Setu AI-provider neutral.
- Keep repo public and secret-free.
- Use Rust unless disproven.
- Add MCP later, not as the initial foundation.

#### `.ai/04-task-board.md`

Purpose: lightweight task tracking.

Suggested sections:

```md
# Task Board

## Active

## Next

## Blocked

## Done

## Parking Lot
```

Tasks should be short and actionable.

#### `.ai/05-risks.md`

Purpose: risk register.

Risk format:

```md
## Risk: title

Severity: low | medium | high
Status: open | mitigated | accepted

Description:

Mitigation:

Owner/Area:
```

Initial risks:

- Accidental secret commit.
- AI client over-permission.
- User exposes Setu beyond localhost.
- Joplin API behavior changes.
- Write operations corrupt notes.
- Conflicts with Joplin sync.
- Public project scope creep.

#### `.ai/06-test-plan.md`

Purpose: testing strategy snapshot.

Should contain:

- Unit test areas.
- Integration test areas.
- Mock Joplin server idea.
- Manual test checklist.
- Security test checklist.

#### `.ai/07-debug-log.md`

Purpose: record non-obvious bugs and fixes.

Use only for issues that future agents might otherwise rediscover.

Format:

```md
## YYYY-MM-DD - Bug title

Symptom:

Cause:

Fix:

Prevention:
```

#### `.ai/08-release-notes.md`

Purpose: draft release notes before they are promoted to `CHANGELOG.md`.

#### `.ai/09-agent-handoff.md`

Purpose: current handoff note for an AI coding agent.

Should contain:

- What to do next.
- Files to inspect first.
- Constraints not to violate.
- Known unresolved questions.

This file should be updated at the end of each meaningful work session.

### Memory update rules

Update `.ai/` when:

- A project decision changes.
- A milestone changes.
- A task is completed or blocked.
- A meaningful risk is discovered.
- A non-obvious bug is fixed.
- A future AI agent would otherwise need to rediscover context.

Do not update `.ai/` when:

- The information is obvious from a small code diff.
- The information belongs in README or docs.
- The information is temporary chat discussion.
- The information contains private data.

### Memory size discipline

The `.ai/` directory should remain compact. As a rough target, keep it under 50 KB unless there is a strong reason.

Old details should be promoted into docs, converted into issues, or removed when stale.

---

## 17. Documentation Structure

Docs should be public, user-friendly, and technically useful.

### `README.md`

Purpose:

- Explain what Setu is.
- Show quick start.
- Show basic examples.
- Link to detailed docs.

### `docs/setup.md`

Purpose:

- Enable Joplin Web Clipper.
- Get token safely.
- Install Setu.
- Configure environment variables.
- Run `setu doctor`.

### `docs/architecture.md`

Purpose:

- Explain architecture layers.
- Explain why Setu wraps Joplin instead of exposing raw API.
- Explain future MCP layer.

### `docs/api.md`

Purpose:

- Document HTTP endpoints.
- Request/response examples.
- Error codes.

### `docs/security.md`

Purpose:

- Explain threat model.
- Explain localhost binding.
- Explain secret handling.
- Explain permission model.

### `docs/joplin-api-notes.md`

Purpose:

- Document relevant Joplin Web Clipper behavior found during implementation.
- Record pagination behavior.
- Record field mappings.
- Record quirks.

### `docs/ai-client-examples.md`

Purpose:

- Explain how to use Setu with ChatGPT, Claude, Gemini, and generic HTTP-capable tools.
- Keep provider-specific details optional and thin.

### `docs/decisions/`

Purpose:

- Store formal ADRs for major decisions.

ADR format:

```md
# ADR 0001: Title

Date: YYYY-MM-DD
Status: accepted | rejected | superseded

## Context

## Decision

## Consequences
```

---

## 18. MVP Definition

The MVP proves that Setu can safely and reliably connect to Joplin and perform basic note operations.

### MVP features

- `setu doctor` command.
- Local config loading.
- Environment variable secret loading.
- Joplin connection check.
- Local HTTP server.
- API key auth.
- `/health` endpoint.
- Note search.
- Note read.
- Note create.
- Note update.
- Note append.
- Notebook list.
- Tag list.
- Clear README setup.
- Security documentation.

### MVP exclusions

- MCP server.
- Attachments.
- Delete/trash operations.
- Semantic search.
- GUI.
- Installer.
- Cloud hosting.
- Multi-user mode.
- AI provider SDKs.

---

## 19. Milestones

### Milestone 0: Planning and Repository Foundation

Deliverables:

- `PROJECT_PLAN.md` detailed.
- README skeleton.
- AGENTS instructions.
- `.gitignore`.
- License.
- Security policy.
- Roadmap.
- Config example.
- `.ai/` memory files.
- Docs skeleton.

Exit criteria:

- A new agent can clone the repo, read project docs, and understand the plan.
- Secret handling rules are documented.
- Scope is clear.

### Milestone 1: Rust Skeleton and Doctor Command

Deliverables:

- Cargo project.
- CLI parser.
- Config loader.
- Joplin client skeleton.
- `setu doctor` command.

Exit criteria:

```text
setu doctor
✓ Configuration loaded
✓ Joplin Web Clipper reachable
✓ Joplin authentication successful
```

### Milestone 2: Local HTTP Server

Deliverables:

- Local server.
- `/health`.
- Auth middleware.
- JSON response envelope.
- Structured errors.

Exit criteria:

- Server runs locally.
- Auth works.
- Health endpoint works.
- Bad requests return useful errors.

### Milestone 3: Notes API

Deliverables:

- Search notes.
- Read note.
- Create note.
- Update note.
- Append note.

Exit criteria:

- Basic note workflow works through Setu.
- No token leakage in logs or responses.
- Tests cover success and error paths.

### Milestone 4: Organization API

Deliverables:

- List notebooks.
- Create notebook.
- List tags.
- Add/remove tags.

Exit criteria:

- Notes can be created into notebooks.
- Tags can be managed safely.

### Milestone 5: Documentation and Public Usability

Deliverables:

- Setup guide.
- API guide.
- Security guide.
- AI client examples.
- Curl examples.

Exit criteria:

- A new user can install and test Setu from docs.

### Milestone 6: v0.1.0 Release

Deliverables:

- Release tag.
- Changelog.
- Built binaries if practical.
- Basic release notes.

Exit criteria:

- Public release is usable for basic Joplin note operations.

### Milestone 7: MCP Mode

Deliverables:

- MCP server interface.
- Tool definitions for note operations.
- Reuse existing domain layer.
- Docs for MCP clients.

Exit criteria:

- MCP-capable clients can use Setu without relying on raw HTTP prompts.

---

## 20. Testing Strategy

### Unit tests

Cover:

- Config parsing.
- Environment variable loading.
- Request validation.
- Response envelope generation.
- Error mapping.
- Domain model conversion.

### Integration tests

Cover:

- HTTP endpoint behavior.
- Auth required/failed/success.
- Mock Joplin server responses.
- Pagination behavior.
- Joplin error mapping.

### Manual tests

Cover:

- Real Joplin Web Clipper connection.
- `setu doctor`.
- Create test note.
- Search test note.
- Append to test note.
- Update test note.
- Tag test note.

### Security tests

Cover:

- Token not logged.
- Token not returned in API response.
- Missing API key rejected.
- Wrong API key rejected.
- Non-localhost bind warning.
- Example config contains no real token.

---

## 21. CLI Design

Initial CLI commands:

```text
setu doctor
setu serve
setu config check
setu joplin check
```

Future CLI commands:

```text
setu note search "query"
setu note read <id>
setu note create --title "Title" --body-file note.md
setu note append <id> --body-file append.md
```

CLI output should be human-friendly. HTTP output should be machine-friendly.

---

## 22. AI Usage Model

Setu should support multiple levels of AI integration.

### Level 1: Human-mediated

AI gives the user a curl command or CLI command. User runs it.

Useful for clients that cannot call local tools.

### Level 2: HTTP tool access

AI client can call local HTTP endpoints directly.

Useful for tool-enabled clients.

### Level 3: MCP mode

Setu exposes proper MCP tools.

Useful for MCP-capable clients.

### Level 4: Future client helpers

Optional thin wrappers may be provided for specific environments, but core must remain provider-neutral.

---

## 23. API Safety Rules for AI Assistants

AI assistants should be guided to:

- Search before creating duplicate notes.
- Read before updating a note.
- Prefer append over replace when preserving history matters.
- Avoid destructive actions.
- Confirm with the user before broad updates.
- Never request or expose tokens in note content.
- Never store private credentials inside Joplin notes via Setu.

These are behavioral guidelines, not all enforceable in code, but Setu should make safe behavior easier than unsafe behavior.

---

## 24. Release Strategy

Use semantic versioning.

### Suggested version path

```text
v0.1.0 - Local HTTP MVP for notes
v0.2.0 - Notebooks and tags
v0.3.0 - Better CLI and examples
v0.4.0 - Attachments/resources if needed
v0.5.0 - MCP preview
v1.0.0 - Stable API and documented compatibility
```

### Changelog

Use `CHANGELOG.md` with sections:

```md
## [Unreleased]

### Added
### Changed
### Fixed
### Security
```

---

## 25. Contribution Model

The project should welcome contributions while protecting scope.

### Contribution rules

- Keep Setu Joplin-focused.
- No secrets in issues, PRs, or tests.
- Add tests for behavior changes.
- Update docs for user-visible changes.
- Record major decisions in ADRs.
- Avoid new dependencies without justification.

### Pull request checklist

- [ ] No secrets or private data included.
- [ ] Tests added or updated.
- [ ] Docs updated if behavior changed.
- [ ] Error handling considered.
- [ ] Security impact considered.
- [ ] `.ai/` updated only if useful for future agents.

---

## 26. AGENTS.md Intent

`AGENTS.md` should instruct AI coding agents how to work in this repo.

It should include:

- Read order.
- Scope constraints.
- Secret rules.
- Coding standards.
- Testing expectations.
- Memory update rules.

Suggested agent read order:

1. `AGENTS.md`.
2. `PROJECT_PLAN.md`.
3. `.ai/00-project-index.md`.
4. Relevant `.ai/` files.
5. Relevant docs.
6. Code and tests.

Important: agents must inspect actual code before making implementation claims.

---

## 27. Open Questions

These should be resolved before or during MVP implementation.

1. Should Setu use Rust from the first commit, or should a tiny Python prototype validate the Joplin flow first?
2. Should all endpoints require Setu API key, including read operations?
3. Should read-only mode be the default?
4. Should note update require an expected `updated_time` to reduce overwrite risk?
5. Should append be the preferred operation for AI-generated additions?
6. Should Setu maintain any local state? Current preference: no.
7. Should MCP be v0.2, v0.5, or post-v1.0?
8. Should packaged binaries be released early or after API stabilizes?
9. Should Windows be the first tested platform because Joplin desktop usage is common there?
10. Should delete/trash be permanently excluded from the core API?

---

## 28. Current Decisions

### Accepted

- Project name is Setu.
- Repository is `vsd9992/setu`.
- Repository is public.
- Setu is only for Joplin integration.
- Setu is local-first.
- Setu is AI-provider neutral.
- Initial integration uses Joplin Web Clipper REST API.
- MCP is future interface work, not the initial foundation.
- Secrets must never be committed.
- Project memory lives in the repo under `.ai/`.
- `.ai/` is compact engineering memory, not a transcript archive.
- Code, tests, and docs outrank `.ai/` memory.

### Proposed

- Rust as implementation language.
- API key required for all non-health endpoints.
- Delete/trash excluded from MVP.
- Append preferred over replace for AI-generated updates.

---

## 29. Immediate Next Step

Create the repository foundation files:

```text
README.md
AGENTS.md
CONTRIBUTING.md
SECURITY.md
ROADMAP.md
CHANGELOG.md
LICENSE
.gitignore
.editorconfig
config.example.toml
docs/
.ai/
```

Then spin off a separate development project/session that uses this repo as the source of truth and starts implementation from Milestone 0.

A fresh agent should be able to start with this instruction:

```text
Clone https://github.com/vsd9992/setu. Read AGENTS.md, PROJECT_PLAN.md, and .ai/00-project-index.md first. Follow the project memory rules. Do not commit secrets. Start with Milestone 0 repository foundation, then proceed toward setu doctor.
```
