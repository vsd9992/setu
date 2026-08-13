# ChatGPT-first local MCP integration

## Decision

Build local stdio MCP as the first AI adapter and test ChatGPT first through OpenAI Secure MCP Tunnel. Add Claude and Gemini in later, independently verified phases. Do not build a public Setu endpoint in MVP.

## Basis

The user approved stdio MCP and a ChatGPT-first rollout. Official OpenAI documentation states that ChatGPT developer-mode MCP connections require public HTTPS or Secure MCP Tunnel; the tunnel can reach a configured local stdio or HTTP MCP server.

## Why

This provides a practical ChatGPT test path while keeping Joplin and Setu local and avoiding a publicly reachable Setu service.

## Important consequences/constraints

- Using ChatGPT still sends deliberately returned note data to the cloud client and must be disclosed.
- Tunnel availability depends on the user's account/workspace policy and must be verified.
- Public plugin submission is a separate architecture requiring a stable public HTTPS endpoint, authentication, operations, and approval.
- Support claims are made only for tested client/version/platform combinations.
