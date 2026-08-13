# Guard note mutations with optimistic concurrency

## Decision

Every append or update requires a version observed during a prior read. Setu rechecks current state and fails without writing when the version is stale.

## Basis

The user approved conflict protection. Joplin may be edited locally or synchronized between the read and write.

## Why

Append is not intrinsically safe, and blind replacement can silently destroy concurrent changes.

## Important consequences/constraints

- Exact Joplin version fields and race behaviour must be verified before implementation.
- A conflict response must contain enough sanitized information to prompt a reread without returning unrelated note data.
- No force-write bypass is included in MVP.
