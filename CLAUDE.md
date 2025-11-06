# Development Guide

Technical documentation for working on this Zed extension for Mojo.

## Architecture

### Components

- **Tree-sitter grammar** (`grammar.js`, `src/scanner.c`) - Syntax parsing
- **Rust extension** (`src/lib.rs`) - LSP integration via Zed extension API
- **Language config** (`languages/mojo/`) - Zed-specific configuration
- **Syntax queries** (`queries/`) - Highlighting, indentation, navigation

### LSP Approach

The extension detects existing `mojo-lsp-server` installations rather than auto-installing.

**Rationale:**
- Zed extensions run in WASM (no shell command execution)
- Mojo distributed via Conda/pip (not standalone binaries)
- Respects user's environment management preferences

**Search paths (in order):**
1. `~/.pixi/bin/mojo-lsp-server` (Pixi global)
2. `~/.local/lib/python3.X/site-packages/max/bin/mojo-lsp-server` (pip/uv)
3. `~/.modular/pkg/packages.modular.com_mojo/bin/mojo-lsp-server` (legacy)
4. System PATH

## Building

### Prerequisites

Install tree-sitter CLI (one-time):

```bash
cargo install tree-sitter-cli
```

### Build Commands

```bash
tree-sitter generate              # Generate parser from grammar.js
cargo build --release             # Build Rust extension
```

### Complete Build

```bash
tree-sitter generate && cargo build --release
```

### Testing Grammar

```bash
tree-sitter parse test.mojo       # Test parsing
tree-sitter test                  # Run grammar tests
```

## Testing

### Install in Zed

1. Open Zed
2. Cmd/Ctrl+Shift+P → "Install Dev Extension"
3. Select this directory

### Verify

- Open a `.mojo` file
- Check syntax highlighting
- Verify LSP features (hover, completion, diagnostics)

## Grammar Development

### Target Syntax

- Mojo v0.25.6+ exclusively (no legacy support)
- Core features: `fn`/`def`, argument conventions (`mut`, `owned`, `ref`, `out`, `read`)
- Structs, traits, basic generics
- Simplified approach: working simple rules > broken complex rules

### Workflow

1. Edit `grammar.js`
2. Run `tree-sitter generate`
3. Test with `tree-sitter parse examples.mojo`
4. Update `queries/highlights.scm` if needed
5. Test in Zed

### Common Issues

- **Conflicts**: Use `conflicts` section or increase precedence
- **External tokens**: Ensure `externals` match `src/scanner.c`
- **Test files**: Use examples from `external/modular/examples/mojo/`

## File Structure

```
.
├── grammar.js                  # Tree-sitter grammar definition
├── src/
│   ├── lib.rs                  # Rust extension (LSP integration)
│   ├── scanner.c               # External scanner (indentation)
│   └── parser.c                # Generated parser (don't edit)
├── queries/
│   ├── highlights.scm          # Syntax highlighting
│   ├── indents.scm             # Indentation rules
│   └── tags.scm                # Symbol navigation
├── languages/mojo/
│   ├── config.toml             # Zed language configuration
│   └── brackets.scm            # Bracket matching
├── extension.toml              # Extension metadata
└── external/modular/           # Official Mojo reference (submodule)
```

## Reference Materials

### Official Mojo Sources

- `external/modular/examples/mojo/` - Modern syntax examples
- `external/modular/mojo/stdlib/` - Standard library
- `external/modular/mojo/docs/` - Documentation and changelog

### Modern Mojo v0.25.6 Patterns

```mojo
# Argument conventions
fn process(mut self, out result: Int, owned data: String, ref config: Config):
    pass

# Struct with ownership
struct Point:
    var x: Int
    var y: Int

    fn __init__(out self, x: Int, y: Int):
        self.x = x
        self.y = y

# Transfer ownership
var tmp = value^
```

## Troubleshooting

### Grammar Generation Fails

- Check for undefined rules or circular references
- Verify `conflicts` section
- Ensure `externals` match scanner
- Simplify complex rules

### LSP Not Working

- Verify `mojo-lsp-server` exists: `which mojo-lsp-server`
- Check paths in `src/lib.rs`
- Test LSP manually: `mojo-lsp-server --help`
- Check Zed logs for errors

### Syntax Highlighting Issues

- Update `queries/highlights.scm` after grammar changes
- Verify query syntax matches grammar rule names
- Check Zed console for query parsing errors

## Git Workflow

### Submodule Management

```bash
# Update to latest Modular sources
git submodule update --remote external/modular

# Check current version
cd external/modular && git log -1 --oneline
```

### Commit Guidelines

- Grammar changes: Update grammar commit hash in `extension.toml`
- Test before committing: `tree-sitter generate` must succeed
- Use examples from `external/modular/` for testing
