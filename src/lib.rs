use zed_extension_api::{self as zed, Result};
use std::fs;
use std::env;

struct MojoExtension;

impl MojoExtension {
    /// Find mojo-lsp-server by checking user's existing installations
    ///
    /// This extension does NOT auto-install Mojo because:
    /// 1. Zed extensions run in WASM and cannot execute shell commands
    /// 2. Mojo is distributed via Conda/pip, not as standalone binaries
    /// 3. Users should manage their Python/Mojo environment with their preferred tool
    ///
    /// Supported installation methods (user must install manually):
    /// - pip: `pip install mojo` or `uv pip install mojo`
    /// - Pixi: `pixi global install mojo` or project-level `pixi add mojo`
    /// - Legacy Modular CLI (deprecated)
    fn find_language_server_binary(&self) -> Result<String> {
        let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());

        // Check common installation locations in priority order
        let search_paths = vec![
            // 1. Pixi global install (recommended by Modular as of 2025)
            format!("{}/.pixi/bin/mojo-lsp-server", home),

            // 2. pip/uv install in user site-packages (v0.25.6+)
            format!("{}/.local/lib/python3.13/site-packages/max/bin/mojo-lsp-server", home),
            format!("{}/.local/lib/python3.12/site-packages/max/bin/mojo-lsp-server", home),
            format!("{}/.local/lib/python3.11/site-packages/max/bin/mojo-lsp-server", home),

            // 3. Legacy Modular CLI installation (deprecated)
            format!("{}/.modular/pkg/packages.modular.com_mojo/bin/mojo-lsp-server", home),
        ];

        // Try each path
        for path in &search_paths {
            if fs::metadata(path).map_or(false, |m| m.is_file()) {
                return Ok(path.clone());
            }
        }

        // Fallback: Return binary name and let system PATH resolve it
        // This works if user has added mojo-lsp-server to their PATH
        Ok("mojo-lsp-server".to_string())
    }
}

impl zed::Extension for MojoExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary_path = self.find_language_server_binary()?;

        Ok(zed::Command {
            command: binary_path,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(MojoExtension);
