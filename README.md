# dircat

A fast command-line tool that concatenates source files from a directory into a single Markdown document with syntax highlighting. Perfect for sharing codebases with LLMs, creating documentation, or archiving project snapshots.

## Features

- **Glob pattern matching** - Include files using flexible glob patterns (e.g., `*.rs`, `**/*.js`)
- **Smart exclusions** - Automatically skips common build directories (`target/`, `node_modules/`, etc.)
- **Syntax highlighting** - Automatically detects language from file extensions for proper Markdown code blocks
- **Hidden file filtering** - Skips dot-directories (`.git`, `.vscode`, etc.) by default
- **Sorted output** - Files are sorted alphabetically by path for consistent output

## Installation

### From source

```sh
git clone https://github.com/yourusername/dircat.git
cd dircat
cargo build --release
```

The binary will be available at `target/release/dircat`.

### Using Cargo

```sh
cargo install --path .
```

## Usage

```sh
dircat <directory> <patterns> [--exclude <pattern>...] [--output <file>]
```

### Arguments

| Argument | Description |
|----------|-------------|
| `<directory>` | The root directory to scan |
| `<patterns>` | Comma-separated glob patterns for files to include |
| `--exclude <pattern>` | Additional glob pattern to exclude (can be used multiple times) |
| `--output`, `-o` | Output file path (default: `output.md`) |

### Examples

**Concatenate all Rust files in the current directory:**

```sh
dircat . "*.rs"
```

**Concatenate all JavaScript and TypeScript files:**

```sh
dircat ./src "*.js,*.ts,*.jsx,*.tsx" -o frontend-code.md
```

**Include all Python files but exclude tests:**

```sh
dircat ./myproject "*.py" --exclude "*_test.py" --exclude "test_*.py"
```

**Export an entire web project:**

```sh
dircat ./webapp "*.html,*.css,*.js" --exclude "*.min.js" -o webapp-source.md
```

## Default Exclusions

The following directories and patterns are excluded by default:

- `target/` - Rust build directory
- `node_modules/` - Node.js dependencies
- `__pycache__/` - Python bytecode cache
- `.git/` - Git repository data
- `dist/` - Distribution builds
- `build/` - Build outputs
- `vendor/` - Vendored dependencies
- `*.lock` - Lock files

All hidden directories (starting with `.`) are also skipped.

## Supported Languages

`dircat` automatically applies syntax highlighting hints for these file extensions:

| Languages | Extensions |
|-----------|------------|
| Python | `.py` |
| JavaScript | `.js` |
| TypeScript | `.ts` |
| JSX/TSX | `.jsx`, `.tsx` |
| Rust | `.rs` |
| Go | `.go` |
| Java | `.java` |
| C/C++ | `.c`, `.cpp` |
| C# | `.cs` |
| Ruby | `.rb` |
| PHP | `.php` |
| Kotlin | `.kt` |
| Swift | `.swift` |
| Scala | `.scala` |
| Shell | `.sh`, `.bash`, `.zsh`, `.fish` |
| PowerShell | `.ps1` |
| SQL | `.sql` |
| HTML/CSS | `.html`, `.htm`, `.css`, `.scss`, `.sass`, `.less` |
| Markup | `.md`, `.markdown`, `.rst`, `.tex` |
| Config | `.json`, `.yaml`, `.yml`, `.toml`, `.ini`, `.cfg`, `.conf`, `.xml` |

## Output Format

The generated Markdown file contains each matched file as a section:

```markdown
### path/to/file.rs

​```rust
// file contents here
​```

---

### path/to/another/file.rs

​```rust
// more contents
​```
```

## Use Cases

- **LLM Context** - Quickly package your codebase to share with AI assistants
- **Code Reviews** - Create a single document for offline review
- **Documentation** - Generate code appendices for technical docs
- **Archiving** - Snapshot source code in a readable format

## Acknowledgments

This project is a Rust rewrite of [folkhack](https://github.com/folkhack)'s original Python script.

## License

MIT