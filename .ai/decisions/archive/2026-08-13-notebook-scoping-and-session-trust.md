# Support notebook scoping and temporary session trust

## Decision

Setu supports an optional notebook allowlist across read and write operations. Writes require confirmation by default, with an option to grant temporary trust to selected write capabilities within selected allowed notebooks for the current process session.

## Basis

The user prefers selected-notebook access and wants the option to trust selected operations for a session while retaining high security and privacy.

## Why

Notebook scoping limits disclosure and write impact. Narrow session trust reduces repetitive prompts without turning a convenience choice into permanent broad authorization.

## Important consequences/constraints

- Allowlist checks are server-side and apply to search results, reads, destinations, and mutations.
- Trust grants are explicit, visible, in-memory only, revocable, and expire on process exit.
- Trust is scoped by capability and notebook and cannot bypass validation, bounds, or stale-write protection.
- Full-note reads outside an explicit allowlist require confirmation.
