# Mojo Extension for Zed

Language support for [Mojo](https://docs.modular.com/mojo) in the Zed editor.

## Features

- Syntax highlighting via tree-sitter grammar
- Language Server Protocol (LSP) integration with `mojo-lsp-server`
- Support for `.mojo` and `.🔥` file extensions
- Modern Mojo v0.25.6+ syntax support

## Requirements

- [Mojo](https://docs.modular.com/mojo/manual/get-started/) installed with `mojo-lsp-server`

The extension searches for `mojo-lsp-server` in standard installation locations and your PATH.

## Installation

Clone and install as a development extension:

```bash
git clone --recursive https://github.com/nijaru/zed-mojo.git
cd zed-mojo
npm install && cargo build --release
```

Then in Zed: Cmd/Ctrl+Shift+P → "Install Dev Extension" → select this directory

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
- Verify installation: `which mojo-lsp-server`
- Restart Zed if you just installed Mojo
- Configure custom path in settings (see Configuration section above)
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
