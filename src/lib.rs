use zed_extension_api as zed;

struct XcodeMcpBundle;

impl XcodeMcpBundle {
    /// Check if xcrun mcpbridge is available (Xcode 26.3+)
    fn check_xcode_mcpbridge(&self) -> Result<(), String> {
        let output = zed::Command::new("xcrun")
            .arg("--find")
            .arg("mcpbridge")
            .output();

        match output {
            Ok(output) => {
                if output.status == Some(0) && !output.stdout.is_empty() {
                    Ok(())
                } else {
                    Err(format!(
                        "Xcode 26.3+ is required: `mcpbridge` was not found. \
                         Install/upgrade Xcode, then ensure the correct Xcode is selected via \
                         `xcode-select -p` (or `sudo xcode-select -s /Applications/Xcode.app`)."
                    ))
                }
            }
            Err(e) => Err(format!(
                "Failed to check for Xcode mcpbridge: {}. \
                     Ensure Xcode 26.3+ is installed and selected via xcode-select.",
                e
            )),
        }
    }

    /// Check if npx is available
    fn check_npx(&self) -> Result<(), String> {
        let output = zed::Command::new("npx").arg("--version").output();

        match output {
            Ok(output) => {
                if output.status == Some(0) {
                    Ok(())
                } else {
                    Err("Node.js (npx) is required to run XcodeBuildMCP. \
                         Install Node.js and ensure `npx` is in PATH."
                        .to_string())
                }
            }
            Err(e) => Err(format!(
                "Failed to check for npx: {}. \
                     Install Node.js and ensure `npx` is in PATH.",
                e
            )),
        }
    }

    fn format_context_server_error(&self, id: &str, error: String) -> String {
        let fallback = "Failed to start the context server. Check the prerequisites and try again.";
        if error.trim().is_empty() {
            return format!("{} ({})", fallback, id);
        }
        format!("{} ({})\n\nDetails: {}", fallback, id, error)
    }
}

impl zed::Extension for XcodeMcpBundle {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        context_server_id: &zed::ContextServerId,
        _project: &zed::Project,
    ) -> Result<zed::Command, String> {
        let id = context_server_id.as_ref();

        match id {
            "Xcode-MCP" => {
                // Preflight check for xcode-mcp
                if let Err(e) = self.check_xcode_mcpbridge() {
                    return Err(self.format_context_server_error(id, e));
                }

                Ok(zed::Command {
                    command: "xcrun".to_string(),
                    args: vec!["mcpbridge".to_string()],
                    env: Vec::new(),
                })
            }
            "XcodeBuildMCP" => {
                // Preflight check for xcodebuildmcp
                if let Err(e) = self.check_npx() {
                    return Err(self.format_context_server_error(id, e));
                }

                let node_path = zed::node_binary_path().map_err(|e| {
                    self.format_context_server_error(
                        id,
                        format!("Failed to resolve Node.js binary path: {}", e),
                    )
                })?;
                let npx_path = node_path.replace("/node", "/npx");

                Ok(zed::Command {
                    command: npx_path,
                    args: vec![
                        "-y".to_string(),
                        "xcodebuildmcp@latest".to_string(),
                        "mcp".to_string(),
                    ],
                    env: Vec::new(),
                })
            }
            _ => {
                Err(self
                    .format_context_server_error(id, format!("Unknown context server id: {}", id)))
            }
        }
    }
}

zed::register_extension!(XcodeMcpBundle);
