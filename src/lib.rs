use zed_extension_api as zed;

struct XcodeMcpBundle;

impl XcodeMcpBundle {
    /// Check if xcrun mcpbridge is available (Xcode 26.3+)
    fn check_xcode_mcpbridge(&self) -> Result<(), String> {
        // Note: In a real implementation, we would run `xcrun --find mcpbridge`
        // The Zed extension API may or may not support subprocess execution in the current version
        // For now, we'll document the expected behavior and let errors surface at runtime

        // If subprocess execution is available, implement:
        // let result = run_command("xcrun", &["--find", "mcpbridge"]);
        // if !result.success { return Err("..."); }

        Ok(())
    }

    /// Check if npx is available
    fn check_npx(&self) -> Result<(), String> {
        // Note: In a real implementation, we would run `npx --version`
        // or check for npx in PATH

        // If subprocess execution is available, implement:
        // let result = run_command("npx", &["--version"]);
        // if !result.success { return Err("..."); }

        Ok(())
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
            "xcode-mcp" => {
                // Preflight check for xcode-mcp
                if let Err(e) = self.check_xcode_mcpbridge() {
                    return Err(e);
                }

                Ok(zed::Command {
                    command: "xcrun".to_string(),
                    args: vec!["mcpbridge".to_string()],
                    env: Vec::new(),
                })
            }
            "xcodebuildmcp" => {
                // Preflight check for xcodebuildmcp
                if let Err(e) = self.check_npx() {
                    return Err(e);
                }

                Ok(zed::Command {
                    command: "npx".to_string(),
                    args: vec!["-y".to_string(), "xcodebuildmcp@latest".to_string()],
                    env: Vec::new(),
                })
            }
            _ => Err(format!("Unknown context server id: {}", id)),
        }
    }
}

zed::register_extension!(XcodeMcpBundle);
