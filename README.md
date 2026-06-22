# zeroheight plugin for Cursor

Connect Cursor to your [zeroheight](https://zeroheight.com) design system. This
plugin registers the zeroheight MCP server so Cursor's agent can pull components,
patterns, usage guidance, and design tokens directly from your styleguide.

## What's included

- **MCP server** (`mcp.json`) — connects to the hosted zeroheight MCP server at
  `https://mcp.zeroheight.com/mcp`.
- **Rule** (`rules/zeroheight.mdc`) — tells the agent to treat the styleguide as
  the source of truth for UI work and to look up components and tokens before
  writing code.

## Authentication

The hosted MCP server uses OAuth. The first time the server is used, Cursor opens
a browser window to sign in to zeroheight and authorize access — no tokens or
environment variables to configure.

## Installation

Install from the Cursor plugin store, or add it manually by pointing Cursor at this
repository.

## Usage

Once connected, ask the agent about your design system, for example:

- "What button variants are available in our design system?"
- "Use our design tokens for the spacing on this card."
- "Find the documentation page for our modal component."
