# Terminate Setu development

## Decision

Stop Setu development and archive the repository as an unsupported prototype. Use Joplin 3.6's maintained native MCP server for connecting AI tools to Joplin.

## Basis

The user's product goal was to connect AI tools with Joplin, not to build a competing security product. During implementation, official Joplin documentation and the installed Joplin 3.6.15 instance established that Joplin now includes a native HTTP MCP server with individually enabled search, read, notebook, tag, create, update, and trash tools.

## Why

Continuing would duplicate upstream functionality and create avoidable maintenance, compatibility, and security burden. Potential policy improvements such as notebook scoping do not justify a separate product without demonstrated demand.

## Important consequences/constraints

- No further feature implementation, release, deployment, tunnel creation, or public endpoint work is planned.
- Preserve the prototype and documentation; do not present them as supported software.
- Do not delete repository history or local work as part of closure.
- Reopening requires new evidence that native Joplin MCP cannot meet a concrete requirement and an explicit user decision.
