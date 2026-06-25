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

## Codex

Connects the [Codex CLI](https://developers.openai.com/codex/) to zeroheight via
a plugin.

What's included:

- **MCP server** (`plugins/zeroheight-mcp/.mcp.json`): the connection to
  zeroheight. Codex reads the server's own guidance, so it treats your
  styleguide as the source of truth when working on UI.

Add this repository as a plugin marketplace, then install the plugin:

```
codex plugin marketplace add zeroheight/ai-plugins
codex plugin add zeroheight-mcp@zeroheight
```

Then sign in to zeroheight (this opens a browser):

```
codex mcp login zeroheight
```

## Claude Code

Connects Claude Code to zeroheight via a plugin. The first time you use it,
Claude Code opens a browser so you can sign in.

What's included:

- **MCP server** (`claude-code/.mcp.json`): the connection to zeroheight. Claude
  Code surfaces the server's own guidance automatically, so it treats your
  styleguide as the source of truth when working on UI.

Add this repository as a plugin marketplace, then install the plugin:

```
/plugin marketplace add zeroheight/ai-plugins
/plugin install zeroheight-mcp@zeroheight
```

## Usage

Once connected, just ask the AI about your design system:

- "What button variants are available in our design system?"
- "Use our design tokens for the spacing on this card."
- "Find the documentation page for our modal component."
