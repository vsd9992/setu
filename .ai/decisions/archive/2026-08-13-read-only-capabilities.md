# Default to read-only capabilities

## Decision

Setu starts read-only. Note creation and note mutation are separate, explicitly enabled capabilities; later notebook and tag writes also receive separate capabilities.

## Basis

The user approved the proposed authorization profile and made security/privacy the highest priority.

## Why

Authentication identifies or admits a caller but does not by itself limit what the caller may do. Explicit capabilities reduce accidental and compromised-client impact.

## Important consequences/constraints

- Disabled capabilities must not be advertised as available tools where the adapter permits dynamic tool exposure.
- Server-side authorization is mandatory even when a client offers confirmations or safety annotations.
- Consequential writes require a user confirmation boundary by default. Explicit session-scoped trust may suppress repeated confirmations only within its narrow approved scope.
