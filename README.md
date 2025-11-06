# Zed Mojo Extension

A modern [Zed](https://zed.dev) editor extension providing comprehensive support for the [Mojo](https://docs.modular.com/mojo) programming language (v0.25.6+).

## Features

### 🎨 **Syntax Highlighting**
- Modern Mojo v0.25.6+ keywords and syntax
- Argument conventions: `mut`, `owned`, `ref`, `out`, `read`
- Function definitions: `fn` and `def`
- Struct and trait definitions
- Type annotations and generics

### 🔧 **Language Server Integration**
- **LSP Support** via Magic platform (`mojo-lsp-server`)
- **Auto-completion** for Mojo APIs and standard library
- **Error diagnostics** and type checking
- **Code navigation** (go to definition, find references)

### 📁 **File Support**
- `.mojo` files
- `.🔥` files (fire emoji extension)
- Proper file type recognition and language switching

### 🌳 **Tree-sitter Grammar**
- Clean, Mojo-first grammar (not Python-based)
- Support for modern argument conventions
- Struct and trait parsing
- Function parameter parsing with ownership semantics

## Installation

### Prerequisites

**IMPORTANT**: This extension does **NOT** auto-install Mojo. You must install Mojo first using one of the methods below.

- **Zed Editor** (latest version)
- **Mojo v0.25.6+** with `mojo-lsp-server` - Choose one installation method:
  - **Recommended**: `pixi global install mojo` (isolated, modern)
  - **Alternative**: `uv pip install mojo` or `pip install mojo`
  - **Legacy**: Modular CLI (deprecated)

### Install Extension

#### Option 1: Development Installation
1. Clone this repository:
   ```bash
   git clone --recursive https://github.com/nijaru/zed-mojo.git
   cd zed-mojo
   ```

2. Build the extension:
   ```bash
   npm install
   cargo build --release
   ```

3. Install in Zed:
   - Open Zed editor
   - Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux/Windows)
   - Type "Install Dev Extension"
   - Select this directory

#### Option 2: Manual Installation
```bash
# Copy to Zed extensions directory
cp -r . ~/.config/zed/extensions/mojo/
```

## Usage

### Quick Start
1. **Create a new Mojo file**: `hello.mojo`
2. **Write modern Mojo code**:
   ```mojo
   fn main():
       var message = "Hello, Mojo!"
       print(message)

   struct Point:
       var x: Int
       var y: Int
       
       fn __init__(out self, x: Int, y: Int):
           self.x = x
           self.y = y
       
       fn distance(self, mut other: Point) -> Float64:
           var dx = self.x - other.x
           var dy = self.y - other.y
           return (dx * dx + dy * dy) ** 0.5
   ```

3. **Enjoy syntax highlighting and LSP features!**

### Modern Mojo Syntax Supported

#### Argument Conventions
```mojo
fn process_data(
    mut buffer: Buffer,    # Mutable reference
    owned data: String,    # Takes ownership
    ref config: Config,    # Immutable reference  
    out result: Int,       # Output parameter
    read flags: Flags      # Read-only access
):
    pass
```

#### Struct Definitions
```mojo
struct MyStruct[T: Movable]:
    var data: T
    
    fn __init__(out self, owned value: T):
        self.data = value^  # Transfer ownership
```

#### Trait Definitions
```mojo
trait Drawable:
    fn draw(self):
        pass
        
    fn area(self) -> Float64:
        pass
```

## Language Server Features

The extension integrates with the official Mojo LSP server via the Magic platform:

- **Diagnostics**: Real-time error checking and warnings
- **Completion**: Smart auto-completion for APIs and symbols
- **Navigation**: Go to definition, find references, symbol outline
- **Formatting**: Code formatting and style suggestions

## Setup: Install Mojo First

**The extension requires Mojo to be installed on your system.** Choose the installation method that best fits your workflow:

### Option 1: Pixi (Recommended) ⭐

Pixi is the officially recommended tool by Modular (as of May 2025). It provides isolated environments without polluting your global Python installation.

```bash
# Install Pixi
curl -fsSL https://pixi.sh/install.sh | bash

# Install Mojo globally
pixi global install mojo

# Verify installation
mojo --version
mojo-lsp-server --help
```

**Why Pixi?**
- ✅ Isolated environments (doesn't pollute global pip)
- ✅ Official Modular recommendation
- ✅ Automatically installs `mojo-lsp-server`
- ✅ Extension auto-detects at `~/.pixi/bin/mojo-lsp-server`

### Option 2: uv/pip (Python package)

If you prefer Python package managers:

```bash
# Using uv (faster, recommended for Python projects)
uv pip install mojo

# Or using pip
pip install mojo

# The LSP server will be at:
# ~/.local/lib/python3.X/site-packages/max/bin/mojo-lsp-server

# Optionally add to PATH:
export PATH="$HOME/.local/lib/python3.13/site-packages/max/bin:$PATH"

# Verify installation
mojo-lsp-server --help
```

### Option 3: Legacy Modular CLI (Deprecated)

**Not recommended** - Modular CLI is deprecated. Use Pixi instead.

---

### Verification

After installation, verify `mojo-lsp-server` is accessible:

```bash
# Check if mojo-lsp-server is found
which mojo-lsp-server

# Or try running it
mojo-lsp-server --help
```

The extension automatically searches these locations:
1. `~/.pixi/bin/mojo-lsp-server` (Pixi global)
2. `~/.local/lib/python3.X/site-packages/max/bin/mojo-lsp-server` (pip/uv)
3. `~/.modular/pkg/packages.modular.com_mojo/bin/mojo-lsp-server` (legacy)
4. System `PATH`

## Grammar Development

This extension uses a **clean, Mojo-first tree-sitter grammar** designed specifically for modern Mojo syntax, rather than adapting Python grammars.

### Key Grammar Features:
- **Modern argument conventions** as first-class syntax
- **Ownership transfer operators** (`^`)
- **Generic type syntax** with constraints
- **Struct and trait definitions** with proper inheritance
- **Minimal conflicts** for better parsing performance

## Contributing

### Development Setup
1. **Clone with submodules**:
   ```bash
   git clone --recursive https://github.com/nijaru/zed-mojo.git
   ```

2. **Install development dependencies**:
   ```bash
   npm install                 # Tree-sitter dependencies
   cargo build --release       # Rust extension
   ```

3. **Test grammar changes**:
   ```bash
   tree-sitter generate       # Regenerate parser
   tree-sitter test           # Run grammar tests
   tree-sitter parse file.mojo # Test specific files
   ```

### Project Structure
```
zed-mojo/
├── extension.toml          # Zed extension configuration
├── Cargo.toml + src/       # Rust LSP integration
├── grammar.js              # Tree-sitter grammar
├── queries/                # Syntax highlighting rules
├── languages/mojo/         # Language configuration
├── external/modular/       # Official Mojo reference (submodule)
└── docs/                   # Documentation
```

## Compatibility

- **Mojo Language**: v0.25.6+ (modern syntax)
- **Zed Editor**: Latest stable version
- **Platforms**: macOS, Linux, Windows
- **Installation**: pip, Pixi, or Magic CLI
- **LSP**: Mojo LSP server via Magic platform

## Troubleshooting

### Common Issues

**LSP not working?**
- **First**: Verify Mojo is installed: `mojo-lsp-server --help`
- **Check installation**: `which mojo-lsp-server` to see the path
- **Install Mojo** if not found: `pixi global install mojo` (recommended)
- **Restart Zed** after installing Mojo
- **Check Zed logs** (Cmd/Ctrl+Shift+P → "Zed: Open Log") for LSP errors
- **Manual override**: Set custom path in Zed settings:
  ```json
  {
    "lsp": {
      "mojo-lsp": {
        "binary": {
          "path": "/path/to/mojo-lsp-server"
        }
      }
    }
  }
  ```

**Syntax highlighting missing?**
- Verify the extension is properly installed
- Check file extension is `.mojo` or `.🔥`
- Try reloading the Zed window

**Grammar parsing errors?**
- This is expected for some complex syntax (work in progress)
- Basic Mojo syntax should work correctly
- Report issues with minimal reproduction cases

## License

MIT License - see [LICENSE](LICENSE) for details.

## Related Projects

- **[Modular Platform](https://github.com/modular/modular)** - Official Mojo implementation
- **[Tree-sitter](https://tree-sitter.github.io/)** - Parser generator used for syntax highlighting
- **[Zed](https://zed.dev)** - The collaborative code editor

---

**Built with ❤️ for the Mojo community**