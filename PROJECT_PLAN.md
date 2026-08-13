# Setu Project Plan

> Setu is a lightweight local gateway that lets AI assistants work with Joplin through Joplin's Web Clipper REST API.

## 1. Project Identity

**Project name:** Setu  
**Repository:** https://github.com/vsd9992/setu  
**Primary purpose:** Connect Joplin with AI assistants such as ChatGPT, Claude, Gemini, and other HTTP-capable or MCP-capable clients.  
**Project type:** Public open-source GitHub project.  
**Initial platform target:** Windows desktop users running Joplin Desktop with Web Clipper enabled.  
**Expansion target:** Cross-platform local gateway for Windows, Linux, and macOS.

Setu means bridge. That is the whole point. The app bridges AI assistants and Joplin without becoming a giant automation monster, because apparently every useful tool eventually tries to become an operating system if nobody stops it.

## 2. Core Mission

Setu should make it safe and practical for an AI assistant to:

- Search Joplin notes.
- Read selected notes.
- Create new notes.
- Update existing notes.
- Append content to existing notes.
- Work with notebooks and tags.
- Help users maintain a structured knowledge base.

Setu should stay focused on Joplin only.

## 3. Non-Goals

Setu is **not**:

- A replacement for Joplin.
- A cloud note service.
- A general automation platform.
- A personal data warehouse.
- A Git, Docker, ERP, calendar, or email tool.
- Tied to OpenAI, Anthropic, Google, or any specific AI provider.
- A place to store secrets, API keys, tokens, or private user data.

## 4. Critical Security Rule

This is a **public GitHub repository**.

Therefore:

- No Joplin Web Clipper token must ever be committed.
- No AI provider API key must ever be committed.
- No user notes, private data, exported notebooks, or personal Joplin content must ever be committed.
- No `.env`, `config.toml`, local database, logs, or generated files containing secrets must be committed.
- Only safe templates such as `.env.example` or `config.example.toml` may be committed.
- The app must load secrets from local environment variables, local ignored config files, or explicit runtime input.

If a contributor accidentally commits a secret, the required response is:

1. Revoke the secret immediately.
2. Remove it from the repository history if needed.
3. Add or update ignore rules and documentation.
4. Treat the leaked secret as compromised forever.

No drama, just containment. Software has already created enough avoidable disasters.

## 5. Target Users

Setu is for users who:

- Use Joplin as their knowledge base.
- Want ChatGPT, Claude, Gemini, or another AI assistant to help manage notes.
- Prefer local-first tools.
- Do not want to expose their entire note vault to a cloud service.
- Want controlled, explicit AI access to Joplin.

## 6. Operating Model

Initial architecture:

```text
AI Assistant
     |
     v
Setu local server
     |
     v
Joplin Web Clipper REST API
     |
     v
Joplin Desktop vault
```

Joplin Web Clipper is expected to run locally, usually at:

```text
http://127.0.0.1:41184
```

Setu should run locally, for example:

```text
http://127.0.0.1:9876
```

The exact port should be configurable.

## 7. First Stable Interface

Setu should expose a clean local HTTP API. It should not simply mirror Joplin's raw API.

Initial endpoints:

```text
GET  /health
POST /note/search
POST /note/read
POST /note/create
POST /note/update
POST /note/append
POST /notebook/list
POST /notebook/create
POST /tag/list
POST /tag/add
```

The public interface should stay simple and stable even if Joplin's internal API changes.

## 8. AI Client Compatibility

Setu should be AI-provider neutral.

Supported interaction styles should eventually include:

- Plain local HTTP calls.
- MCP server mode.
- CLI mode.
- Optional copy-paste friendly mode for AI clients that cannot call local tools directly.

Setu must not require an OpenAI, Anthropic, Google, or other AI SDK.

## 9. Technology Direction

Preferred implementation language: **Rust**.

Reasoning:

- Good for small local services.
- Strong binary distribution story.
- Excellent error handling.
- Good performance with low overhead.
- Suitable for public open-source tooling.

Likely initial stack:

```text
axum       - local HTTP server
tokio      - async runtime
reqwest    - Joplin API client
serde      - JSON serialization
toml       - config parsing
tracing    - logging
clap       - CLI commands
anyhow     - application error handling
thiserror  - library error types
```

Keep dependencies boring and necessary. Nobody needs a dependency forest to pass JSON between two local apps.

## 10. Repository Structure

Proposed initial repository structure:

```text
setu/
├── README.md
├── PROJECT_PLAN.md
├── AGENTS.md
├── LICENSE
├── .gitignore
├── config.example.toml
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── config.rs
│   ├── server/
│   │   ├── mod.rs
│   │   ├── routes.rs
│   │   └── auth.rs
│   ├── joplin/
│   │   ├── mod.rs
│   │   ├── client.rs
│   │   ├── notes.rs
│   │   ├── notebooks.rs
│   │   └── tags.rs
│   └── models/
│       ├── mod.rs
│       ├── note.rs
│       ├── notebook.rs
│       └── tag.rs
├── docs/
│   ├── setup.md
│   ├── api.md
│   ├── security.md
│   └── ai-client-examples.md
└── .ai/
    ├── 00-project-index.md
    ├── 01-decisions.md
    ├── 02-task-board.md
    ├── 03-risks.md
    └── 04-debug-log.md
```

The `.ai/` folder is for project memory only. It must contain project decisions, tasks, risks, and implementation notes. It must not contain secrets or private user note content.

## 11. Project Memory Rules

The GitHub repository is the project root and memory storage.

Project memory files should be stored in `.ai/` and must follow these rules:

- Keep memory short and factual.
- Record decisions, not every conversation.
- Record why a decision was made, not only what was chosen.
- Source code, tests, and docs are the implementation truth.
- `.ai/` files guide future work but must not override code or tests.
- Never store API keys, Joplin tokens, credentials, private note content, or personal data.

Recommended memory files:

```text
.ai/00-project-index.md   - current project map
.ai/01-decisions.md       - technical and product decisions
.ai/02-task-board.md      - active, next, blocked, done
.ai/03-risks.md           - security, usability, compatibility risks
.ai/04-debug-log.md       - major bugs and fixes
```

## 12. Configuration Model

Setu should support a local config file, but the real local config file must be ignored by Git.

Example safe file:

```toml
# config.example.toml

[server]
host = "127.0.0.1"
port = 9876

[joplin]
base_url = "http://127.0.0.1:41184"
token_env = "JOPLIN_TOKEN"

[security]
api_key_env = "SETU_API_KEY"
```

Actual secrets should be loaded from environment variables:

```text
JOPLIN_TOKEN
SETU_API_KEY
```

Local-only files to ignore:

```text
.env
config.toml
*.local.toml
logs/
tmp/
```

## 13. Security Model

Setu must be local-first and locked down by default.

Initial rules:

- Bind to `127.0.0.1` by default.
- Never bind to `0.0.0.0` unless explicitly configured.
- Require a Setu API key for write operations.
- Keep read operations protected as well unless a user explicitly disables auth.
- Never log secrets.
- Never return the Joplin token in any response.
- Provide clear warnings if the server is exposed beyond localhost.

Future rules:

- Permission profiles: read-only, write-notes, full-notes.
- Per-client API keys.
- Optional allowlist of permitted operations.
- Optional confirmation mode for destructive actions.

## 14. MVP Scope

MVP should do only enough to prove that Setu works reliably.

### MVP Features

- Start local Setu server.
- Read config safely.
- Connect to Joplin Web Clipper API.
- Verify Joplin connection.
- Search notes.
- Read note by ID.
- Create note.
- Update note.
- Append to note.
- List notebooks.
- List tags.
- Basic API key protection.
- Useful error messages.
- README setup instructions.

### MVP Exclusions

- MCP mode.
- Attachments.
- Semantic search.
- Multi-user support.
- Cloud hosting.
- Joplin sync handling.
- Full note conflict resolution.
- OAuth.
- GUI.

## 15. Milestones

### Milestone 0: Repository Foundation

- Add README.
- Add PROJECT_PLAN.
- Add AGENTS instructions.
- Add `.gitignore`.
- Add license.
- Add config example.
- Add `.ai/` memory files.

### Milestone 1: Joplin API Proof

- Create minimal Rust app.
- Load config.
- Call Joplin `/ping` or equivalent health endpoint.
- Validate token handling.
- Print connection status.

### Milestone 2: Local HTTP Server

- Add local Setu server.
- Add `/health`.
- Add API key middleware.
- Add structured error responses.

### Milestone 3: Notes API

- Implement search.
- Implement read.
- Implement create.
- Implement update.
- Implement append.

### Milestone 4: Organization API

- List notebooks.
- Create notebook.
- List tags.
- Add tag to note.

### Milestone 5: Documentation and Examples

- Document setup for Joplin Web Clipper.
- Document local environment variables.
- Provide curl examples.
- Provide AI prompt examples for ChatGPT, Claude, Gemini.

### Milestone 6: First Public Release

- Tag `v0.1.0`.
- Publish release binaries if practical.
- Add changelog.
- Add security notes.

## 16. API Response Style

Responses should be predictable and boring, the highest compliment one can give infrastructure.

Example success response:

```json
{
  "ok": true,
  "data": {
    "id": "note_id",
    "title": "Example Note"
  }
}
```

Example error response:

```json
{
  "ok": false,
  "error": {
    "code": "JOPLIN_CONNECTION_FAILED",
    "message": "Could not connect to Joplin Web Clipper at http://127.0.0.1:41184"
  }
}
```

## 17. Compatibility Assumptions

Initial development assumptions:

- Joplin Desktop stable version 3.6.x or later.
- Web Clipper enabled.
- User has access to the Web Clipper token.
- Joplin runs locally on the same machine as Setu.
- OneDrive, Dropbox, Nextcloud, or other Joplin sync targets are irrelevant to Setu because Setu talks to the local Joplin app, not the sync backend.

## 18. Naming Notes

The project name is **Setu**.

Possible short descriptions:

- "Local AI gateway for Joplin."
- "A safe bridge between Joplin and AI assistants."
- "Use Joplin with ChatGPT, Claude, Gemini, and other AI tools."

Avoid descriptions that imply cloud hosting or AI-provider lock-in.

## 19. Open Questions

- Should the first release be Rust-only, or should a simpler Node/Python prototype exist first?
- Should MCP be built into Setu v0.2 or deferred until v1.0?
- Should destructive operations such as delete/trash be excluded permanently or guarded behind confirmation mode?
- Should Setu support read-only mode as the default?
- Should Setu offer a CLI-only mode before local HTTP mode?

## 20. Current Decisions

- The project will be public on GitHub.
- The repo `vsd9992/setu` is the project root.
- Project memory will live in the repo, preferably under `.ai/`.
- Setu will focus only on Joplin.
- Setu will be AI-assistant agnostic.
- No secrets, keys, tokens, private notes, or personal user data will be stored in the repo.
- Initial integration will use Joplin Web Clipper REST API, not Joplin MCP, because MCP is not available in the current stable Joplin release.
- MCP can be added later as a Setu interface, not as the foundation of the project.
