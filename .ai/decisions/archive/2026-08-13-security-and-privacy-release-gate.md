# Security and privacy are release gates

## Decision

Security and privacy have priority over feature breadth, convenience, compatibility claims, and release timing. A milestone cannot complete while its applicable security or privacy criteria fail or remain unverified.

## Basis

Setu handles private notes and credentials and can authorize AI-initiated writes. Local-first architecture reduces but does not remove risk, especially when a cloud AI client receives note content.

## Why

Retrofitting authorization, data minimization, and redaction after integrations ship would expose users and create incompatible contracts.

## Important consequences/constraints

- Default deny and least privilege apply to tools, fields, results, and writes.
- Security-negative tests and privacy disclosures are acceptance criteria.
- No public endpoint, hosted relay, destructive operation, or broadened data scope without explicit approval and a revised threat model.
- Compatibility or delivery may be delayed when safe behaviour is not demonstrated.
