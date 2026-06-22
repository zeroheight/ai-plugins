# ai-plugins

Bring your [zeroheight](https://zeroheight.com) design system into your favourite
AI coding tools. These plugins connect your editor to zeroheight so the AI can
reach for your components, patterns, usage guidance, and design tokens while it
works, straight from your styleguide.

## Cursor

Connects Cursor to zeroheight. The first time you use it, Cursor opens a browser
so you can sign in.

What's included:

- **MCP server** (`cursor/mcp.json`): the connection to zeroheight.
- **Rule** (`cursor/rules/zeroheight.mdc`): nudges the AI to treat your
  styleguide as the source of truth and look things up before writing UI code.

Install it from the Cursor plugin store, or point Cursor at this repository.

## Zed

A Zed extension that runs the zeroheight MCP server for you. See
[`zed/README.md`](zed/README.md) for setup.

## Usage

Once connected, just ask the AI about your design system:

- "What button variants are available in our design system?"
- "Use our design tokens for the spacing on this card."
- "Find the documentation page for our modal component."
