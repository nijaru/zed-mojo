# Mojo Extension for Zed

Language support for [Mojo](https://docs.modular.com/mojo) in the Zed editor.

## Features

- Syntax highlighting via tree-sitter grammar
- Language Server Protocol (LSP) integration with `mojo-lsp-server`
- Support for `.mojo` and `.🔥` file extensions
- Modern Mojo v0.25.6+ syntax support

## Requirements

This extension requires `mojo-lsp-server` to be installed on your system. The extension will search for it in common locations but does not install it automatically.

### Installing Mojo

Choose one of the following installation methods:

**Pixi (Recommended):**
```bash
pixi global install mojo
```

**uv or pip:**
```bash
uv pip install mojo
# or
pip install mojo
```

Verify installation:
```bash
mojo-lsp-server --help
```

## Installation

### From Zed Extensions

Search for "Mojo" in the Zed extensions panel and click Install.

### Development Installation

```bash
git clone --recursive https://github.com/nijaru/zed-mojo.git
cd zed-mojo
npm install && cargo build --release
```

Then use "Install Dev Extension" in Zed (Cmd/Ctrl+Shift+P).

## Configuration

The extension automatically searches for `mojo-lsp-server` in:
1. `~/.pixi/bin/mojo-lsp-server`
2. `~/.local/lib/python3.X/site-packages/max/bin/mojo-lsp-server`
3. `~/.modular/pkg/packages.modular.com_mojo/bin/mojo-lsp-server`
4. System PATH

To override the default search, configure the LSP binary path in your Zed settings:

```json
{
  "lsp": {
    "mojo-lsp": {
      "binary": {
        "path": "/custom/path/to/mojo-lsp-server"
      }
    }
  }
}
```

## Troubleshooting

**LSP server not found:**
- Verify Mojo is installed: `which mojo-lsp-server`
- Restart Zed after installing Mojo
- Check Zed logs: Cmd/Ctrl+Shift+P → "Zed: Open Log"

**Syntax highlighting issues:**
- Verify file extension is `.mojo` or `.🔥`
- Reload window: Cmd/Ctrl+Shift+P → "Reload Window"

## Development

### Building the Grammar

```bash
tree-sitter generate
tree-sitter parse test.mojo
```

### Building the Extension

```bash
cargo build --release
```

### Testing

Open a `.mojo` file in Zed and verify:
- Syntax highlighting appears
- LSP features work (hover, completion, diagnostics)

## License

MIT - See [LICENSE](LICENSE)

## Links

- [Mojo Documentation](https://docs.modular.com/mojo)
- [Modular Platform](https://github.com/modular/modular)
- [Tree-sitter](https://tree-sitter.github.io/)
- [Zed Editor](https://zed.dev)
