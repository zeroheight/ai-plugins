# zeroheight MCP Server for Zed

This extension launches the `@zeroheight/mcp-server` MCP server in Zed.

Credentials are optional. Configure them in Zed's context server settings when
you can't use OAuth:

```json
{
  "context_servers": {
    "mcp-server-zeroheight": {
      "settings": {
        "ZEROHEIGHT_ACCESS_TOKEN": "zhat_abc123",
        "ZEROHEIGHT_CLIENT_ID": "zhci_abc123"
      }
    }
  }
}
```

The same variables can also be provided through `command.env`; blank credential
values are ignored.
