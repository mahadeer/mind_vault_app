## Running MindVault

MindVault can be run as a monolithic application or integrated with external tools.

#### Running as a Monolithic Application

If you're running the `mindvault-api` directly, you would typically start it from its `target/release/` directory:

```bash
./target/release/mindvault-api
```

```bash
./target/release/mindvault-ui
```

You would then interact with it via its API endpoints or the `frontend` UI.

#### Integration with External Tools (MCP Servers)

MindVault is designed to be launched and managed by external tools or workflows configured to interact with "MCP servers." If your external tool expects a configuration to launch and interact with an MCP server, you can configure it to run MindVault's API as follows:

```json
{
  "mcpServers": {
    "mindvault-local": {
      "command": "path/to/target/release/mindvault_mcp"
    }
  }
}
```