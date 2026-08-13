# Target Windows for the first development preview

## Decision

Test and support the first development preview on Windows using the user's local Joplin installation and a dedicated notebook containing only synthetic test data.

## Basis

The user's current environment is Windows and their local Joplin installation is available for controlled testing.

## Why

A single verified platform keeps early compatibility claims evidence-based while the implementation remains portable.

## Important consequences/constraints

- Record the exact Windows and Joplin versions during verification.
- Developer-preview secrets use environment variables.
- Add Windows Credential Manager support before broader Windows usability claims.
- Do not access personal notebooks during development validation.
