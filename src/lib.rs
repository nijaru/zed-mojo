use zed_extension_api::{self as zed, Result};
use std::fs;

struct MojoExtension;

impl MojoExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        // Try to find mojo-lsp-server in various locations

        // 1. First, try using 'mojo-lsp-server' from PATH (simplest)
        // This works if user installed via pip or has .modular/bin in PATH
        let binary_name = "mojo-lsp-server";

        // 2. Check common installation locations if PATH lookup fails
        let possible_paths = vec![
            // Modular CLI installation
            format!("{}/.modular/pkg/packages.modular.com_mojo/bin/mojo-lsp-server",
                    std::env::var("HOME").unwrap_or_default()),
            // pip install in user site-packages (common with uv/pip)
            format!("{}/.local/lib/python3.13/site-packages/max/bin/mojo-lsp-server",
                    std::env::var("HOME").unwrap_or_default()),
            format!("{}/.local/lib/python3.12/site-packages/max/bin/mojo-lsp-server",
                    std::env::var("HOME").unwrap_or_default()),
            format!("{}/.local/lib/python3.11/site-packages/max/bin/mojo-lsp-server",
                    std::env::var("HOME").unwrap_or_default()),
        ];

        // Check if mojo-lsp-server exists in any of the known paths
        for path in &possible_paths {
            if fs::metadata(path).map_or(false, |m| m.is_file()) {
                return Ok(path.clone());
            }
        }

        // If we can't find it in known locations, just return the binary name
        // and let the system PATH resolve it. If it's not found, Zed will
        // show an error message to the user.
        Ok(binary_name.to_string())
    }
}

impl zed::Extension for MojoExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary_path = self.language_server_binary_path(language_server_id, worktree)?;

        Ok(zed::Command {
            command: binary_path,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(MojoExtension);
