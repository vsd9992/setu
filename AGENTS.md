# Setu Repository Instructions

## Orientation

- Read `.ai/00-project-index.md` when orienting or restoring context.
- Start with the minimum context needed for the current task and escalate only when uncertainty requires it.
- After context compaction or loss, re-read this file and the project index.
- Do not re-read unchanged files already reliably read in the current context unless they may have changed.

## Truth and scope

- Intended behaviour comes from the user's current instruction, the approved baseline under `.ai/project/`, and relevant current decisions under `.ai/decisions/current/`.
- Current behaviour comes from source, configuration, schema, tests, and observed runtime behaviour.
- Treat disagreement between intended and implementation truth as a deviation; do not silently change either side.
- Verify file-backed facts and inspect relevant source/config/schema before describing current implementation.
- Work one task at a time. Do not re-plan the project unless the phase or user requires it.
- Do not change scope, requirements, product behaviour, or architecture without approval.
- Do not refactor unrelated code.

## Safety and verification

- Never commit secrets, credentials, tokens, private notes, production data, local account/session files, or sensitive deployment, DNS, SSL, or payment configuration.
- Keep Setu bound to loopback by default; changes to exposure or trust boundaries require explicit approval and security review.
- Never claim a build, test, lint, or check passed unless it was actually run successfully.
- Keep searches, listings, diffs, logs, and test output bounded. Avoid generated files, dependencies, build output, binaries, lockfiles, and large logs unless directly relevant.

## Documentation lifecycle

- Maintain live state in `.ai/00-project-index.md`, roadmap status, active tasks, and active risks.
- Change baseline documents only when approved project truth changes.
- Create task records only when losing non-obvious reasoning would cause meaningful rework; archive them when complete.
- Preserve durable AI memory only when losing it would cause meaningful rework or repeated mistakes.
- Keep the project index synchronized with meaningful task, milestone, decision, and archive changes.

## Commands

- Format check: `cargo fmt --check`
- Compile check: `cargo check`
- Tests: `cargo test`
- Lint: `cargo clippy --all-targets --all-features -- -D warnings`
- Safe unauthenticated diagnostic: `cargo run -- doctor`
