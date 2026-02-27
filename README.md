# Xcode MCP Bundle

A Zed extension that bundles two MCP (Model Context Protocol) servers for Xcode development:

> [!NOTE]
> Xcode 26.3 is now released. Install Xcode 26.3+ and ensure it’s selected via `xcode-select` before enabling the MCP servers.
> Make sure Xcode is opened and MCP Server is available before using through this extension.

- **xcode-mcp**: Xcode 26.3+ MCP tools via `xcrun mcpbridge`
- **xcodebuildmcp**: XcodeBuildMCP via `npx -y xcodebuildmcp@latest`

This extension performs **automated preflight checks** to ensure all prerequisites are met before starting each context server, providing clear error messages if dependencies are missing.

## Installation

Install this extension from the Zed marketplace or by adding it to your Zed configuration.

## What You Get

### xcode-mcp
Exposes Xcode's built-in MCP tools when using Xcode 26.3 or later. This server provides deep integration with Xcode's development environment.

### xcodebuildmcp
Provides XcodeBuildMCP functionality through npm, offering build and test capabilities for Xcode projects.

## Setup

### xcode-mcp Setup

To use the `xcode-mcp` server:

1. Install Xcode 26.3 or later
2. Ensure the correct Xcode is selected via `xcode-select`:
   ```bash
   xcode-select -p
   # Should point to /Applications/Xcode.app or your chosen Xcode installation
   # If needed, set it with: sudo xcode-select -s /Applications/Xcode.app
   ```
3. Open Xcode with your workspace or project
4. In Xcode, go to **Settings → Intelligence → Model Context Protocol**
5. Enable "Xcode Tools"
6. In Zed, open the Agent Panel → Context Servers
7. Enable the `xcode-mcp` server

### xcodebuildmcp Setup

To use the `xcodebuildmcp` server:

1. Install Node.js from [nodejs.org](https://nodejs.org/)
2. Verify `npx` is available:
   ```bash
   npx --version
   ```
3. In Zed, open the Agent Panel → Context Servers
4. Enable the `xcodebuildmcp` server

## Error Messages and Troubleshooting

This extension performs automated preflight checks before starting each context server. If a prerequisite is missing, you'll receive an immediate, clear error message. Here's how to resolve common issues:

### xcode-mcp Errors

#### Error: "Xcode 26.3+ is required: `mcpbridge` was not found"

**Cause**: This error occurs during the extension's automated preflight check. You're using an older version of Xcode, or Xcode is not properly installed, or the wrong Xcode installation is selected.

**Solution**:
1. Verify Xcode 26.3+ is installed:
   ```bash
   xcrun --version
   ```
   This should show version 26.3 or higher.

2. Verify `mcpbridge` is available:
   ```bash
   xcrun --find mcpbridge
   ```
   This should return a path (e.g., `/Applications/Xcode.app/Contents/Developer/usr/bin/mcpbridge`).

3. Check which Xcode is selected:
   ```bash
   xcode-select -p
   ```
   
4. If the wrong Xcode is selected, fix it:
   ```bash
   sudo xcode-select -s /Applications/Xcode.app
   ```

5. Ensure Xcode is running with your project/workspace open before enabling the server.

6. Verify Xcode Tools MCP is enabled in Xcode settings:
   - Open Xcode → Settings → Intelligence → Model Context Protocol
   - Make sure "Xcode Tools" is checked

#### Error: "Failed to start xcode-mcp server"

**Possible causes**:
- Xcode is not running
- Xcode Tools MCP is not enabled in Xcode settings
- Network or IPC issues between Zed and Xcode

**Solutions**:
1. Make sure Xcode is open and has a project/workspace loaded
2. Check that Xcode Tools MCP is enabled in Xcode settings
3. Try restarting Xcode and then re-enabling the server in Zed
4. Check Zed's logs for more detailed error messages

### xcodebuildmcp Errors

#### Error: "Node.js (npx) is required to run XcodeBuildMCP. Install Node.js and ensure `npx` is in PATH"

**Cause**: This error occurs during the extension's automated preflight check. Node.js is not installed, or `npx` is not available in your PATH.

**Solution**:
1. Install Node.js from [nodejs.org](https://nodejs.org/) (LTS version recommended)
   
2. Verify Node.js installation:
   ```bash
   node --version
   ```

3. Verify `npx` is available:
   ```bash
   npx --version
   ```
   This should print a version number.

4. If `npx` is not found, you may need to restart your terminal or shell, or check your PATH:
   ```bash
   echo $PATH
   ```
   Ensure the Node.js bin directory (e.g., `/usr/local/bin` or `/opt/homebrew/bin`) is in your PATH.

5. After fixing PATH, restart Zed and try enabling the server again.

#### Error: "Failed to start xcodebuildmcp server"

**Possible causes**:
- Network connectivity issues (npx needs to download the package)
- npm registry issues
- Temporary download failure

**Solutions**:
1. Test `npx` manually to see the actual error:
   ```bash
   npx -y xcodebuildmcp@latest
   ```

2. Check your internet connection

3. If you see npm registry errors, try clearing npm cache:
   ```bash
   npm cache clean --force
   ```

4. If you're behind a corporate firewall or proxy, you may need to configure npm to use your proxy:
   ```bash
   npm config set proxy http://proxy.company.com:port
   npm config set https-proxy http://proxy.company.com:port
   ```

5. Check Zed's logs for more detailed error messages

### General Issues

#### Server fails to start with no clear error message

**Solution**:
1. Open Zed's logs to see detailed error messages:
   - Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux/Windows)
   - Type "zed: open log"
   - Look for errors related to the context server

2. Try manually running the command in a terminal to see what error occurs:
   ```bash
   # For xcode-mcp
   xcrun mcpbridge
   
   # For xcodebuildmcp
   npx -y xcodebuildmcp@latest
   ```

3. If you find a bug or have an issue that's not covered here, please report it on GitHub:
   https://github.com/tommyming/zed-xcode-mcp-bundle/issues

## Tool Overlap Behavior

Both servers may expose similar build and test capabilities. For best results, you can:

- Enable only the server you need for your current task
- Use different Zed Agent profiles for different workflows

### Suggested Agent Profiles

You can create agent profiles in Zed's settings to quickly switch between server configurations:

**Profile A: "Xcode MCP"** (enable `xcode-mcp` only)
- Best for deep Xcode integration when working directly in Xcode
- Provides access to Xcode's internal tools and analysis

**Profile B: "XcodeBuildMCP"** (enable `xcodebuildmcp` only)
- Best for command-line build and test workflows
- Useful for CI/CD scripts and automation

**Profile C: "iOS Full Stack"** (enable both)
- Best when you want maximum tooling coverage
- Combines Xcode's deep integration with flexible build tools

## Requirements

### For xcode-mcp
- macOS
- Xcode 26.3 or later installed
- Xcode selected via `xcode-select`
- Xcode running with a workspace or project open
- Xcode Tools MCP enabled in Xcode settings

### For xcodebuildmcp
- Node.js installed
- `npx` available in PATH
- Internet connection (for downloading via npx)

## Development

### Preflight Checks

The extension implements automated preflight checks that run before starting each context server:

- **xcode-mcp**: Runs `xcrun --find mcpbridge` to verify Xcode 26.3+ is installed and accessible
- **xcodebuildmcp**: Runs `npx --version` to verify Node.js and npx are available

These checks provide immediate feedback if prerequisites are missing, saving you from debugging runtime errors.

### Building the Extension

```bash
cargo build --release
```

This will create a WebAssembly file that Zed can load.

### Local Testing

1. In Zed, press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux/Windows)
2. Type "zed: install dev extension"
3. Navigate to this repository's directory
4. Select the `extension.toml` file

The extension will be loaded and you can test the context servers.

### Viewing Logs

To view detailed logs for debugging:

1. Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux/Windows)
2. Type "zed: open log"

Look for messages prefixed with `xcode-mcp-bundle` to see extension-specific logs.

## License

Apache-2.0

## Repository

https://github.com/tommyming/zed-xcode-mcp-bundle

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for bugs and feature requests.
