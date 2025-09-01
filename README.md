# kfmtbox 🌸

> A beautiful Rust CLI tool that wraps your text in elegant flower box comments

[![Crates.io](https://img.shields.io/crates/v/kfmtbox.svg)](https://crates.io/crates/kfmtbox)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/koftwentytwo/kfmtbox/workflows/CI/badge.svg)](https://github.com/koftwentytwo/kfmtbox/actions)

Transform your plain text into beautiful, decorative comment boxes that automatically adapt to your programming language's comment syntax.

## ✨ Features

- 🎨 **Beautiful Unicode box drawing** with perfect rectangular formatting
- 🔍 **Smart file type detection** from filename or explicit specification
- 📝 **Intelligent text wrapping** with balanced line lengths
- 🔄 **Existing comment detection** - reformats existing flower boxes
- ⚙️ **YAML configuration** for custom comment styles
- 🚀 **Editor integration** ready for Neovim, VSCode, and more
- 🦀 **Written in Rust** - fast, reliable, and memory-safe

## 🚀 Quick Start

### Installation

#### 🚀 One-Line Install (Recommended)
```bash
# Linux/macOS
curl -sSL https://raw.githubusercontent.com/koftwentytwo/kfmtbox/main/install.sh | bash

# Windows PowerShell
powershell -ExecutionPolicy Bypass -File <(curl -sSL https://raw.githubusercontent.com/koftwentytwo/kfmtbox/main/install.ps1)
```

#### 📦 Package Managers

**Homebrew (macOS/Linux)**
```bash
# Add the tap (after first release)
brew tap koftwentytwo/kfmtbox
brew install kfmtbox

# Or install directly from formula
brew install koftwentytwo/kfmtbox/kfmtbox
```

**Cargo (Rust)**
```bash
cargo install kfmtbox
```

**Arch Linux (AUR)**
```bash
# After AUR package is created
yay -S kfmtbox
```

#### 🔨 From Source
```bash
git clone https://github.com/koftwentytwo/kfmtbox.git
cd kfmtbox
cargo build --release
sudo cp target/release/kfmtbox /usr/local/bin/
```

#### 📥 Manual Download
Visit the [Releases](https://github.com/koftwentytwo/kfmtbox/releases) page to download pre-built binaries for your platform.

### Basic Usage

```bash
echo "Hello, world!" | kfmtbox --filetype rs
```

**Output:**
```
// ╭───────────────╮
//  │Hello, world! │
// ╰───────────────╯
```

## 📖 Examples

### Different Programming Languages

```bash
# Rust
echo "Important note" | kfmtbox --filetype rs

# Python  
echo "TODO: Fix this bug" | kfmtbox --filetype py

# JavaScript
echo "This function does magic" | kfmtbox --filetype js

# SQL
echo "Complex query explanation" | kfmtbox --filetype sql
```

### Text Wrapping

```bash
# Long text automatically wraps and balances
echo "This is a very long comment that will be automatically wrapped into multiple lines with balanced spacing" | kfmtbox --filetype py --max-width 40
```

### Reformatting Existing Comments

```bash
# Reformat existing flower boxes
echo "// ╭─────╮
//  │Old │
// ╰─────╯" | kfmtbox --filename test.rs
```

## 🛠️ Command Line Options

| Option            | Description                           | Example              |
|-------------------|---------------------------------------|----------------------|
| `-f, --filename`  | Detect file type from filename        | `--filename main.rs` |
| `-t, --filetype`  | Specify file type explicitly          | `--filetype py`      |
| `-w, --max-width` | Set maximum line width (default: 120) | `--max-width 80`     |
| `-h, --help`      | Show help information                 | `--help`             |
| `-V, --version`   | Show version information              | `--version`          |

## ⚙️ Configuration

Create a YAML configuration file to customize comment styles:

**macOS:** `~/Library/Application Support/kfmtbox/comments.yaml`  
**Linux:** `~/.config/kfmtbox/comments.yaml`  
**Windows:** `%APPDATA%\kfmtbox\comments.yaml`

```yaml
comment_styles:
  # Standard languages
  rust: "//"
  python: "#"
  javascript: "//"
  typescript: "//"
  go: "//"
  java: "//"
  cpp: "//"
  c: "//"
  php: "//"
  ruby: "#"
  shell: "#"
  vim: "\""
  lua: "--"
  sql: "--"
  yaml: "#"
  toml: "#"
  ini: ";"
  dockerfile: "#"
  makefile: "#"
  cmake: "#"
  
  # Custom file types
  custom: "%%"
  special: "!!"
```

## 🎯 Supported File Types

| Language       | Extensions                      | Comment Style |
|----------------|---------------------------------|---------------|
| **Rust**       | `.rs`                           | `//`          |
| **Python**     | `.py`                           | `#`           |
| **JavaScript** | `.js`                           | `//`          |
| **TypeScript** | `.ts`                           | `//`          |
| **Go**         | `.go`                           | `//`          |
| **Java**       | `.java`                         | `//`          |
| **C/C++**      | `.c`, `.cpp`, `.h`, `.hpp`      | `//`          |
| **C#**         | `.cs`                           | `//`          |
| **PHP**        | `.php`                          | `//`          |
| **Ruby**       | `.rb`                           | `#`           |
| **Shell**      | `.sh`, `.bash`, `.zsh`, `.fish` | `#`           |
| **Vim**        | `.vim`                          | `"`           |
| **Lua**        | `.lua`                          | `--`          |
| **SQL**        | `.sql`                          | `--`          |
| **YAML**       | `.yaml`, `.yml`                 | `#`           |
| **TOML**       | `.toml`                         | `#`           |
| **INI**        | `.ini`, `.cfg`                  | `;`           |
| **Dockerfile** | `Dockerfile`                    | `#`           |
| **Makefile**   | `Makefile`                      | `#`           |
| **CMake**      | `CMakeLists.txt`                | `#`           |

## 🔌 Editor Integration

### Neovim

Add to your `init.vim` or `init.lua`:

```vim
" Format selected text as flower box
vnoremap <leader>fb :!kfmtbox --filename %<CR>

" Language-specific shortcuts
vnoremap <leader>fr :!kfmtbox --filetype rs<CR>
vnoremap <leader>fp :!kfmtbox --filetype py<CR>
vnoremap <leader>fj :!kfmtbox --filetype js<CR>
```

### VSCode

Create a custom task in `.vscode/tasks.json`:

```json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "Format as Flower Box",
            "type": "shell",
            "command": "kfmtbox",
            "args": ["--filename", "${file}"],
            "group": "build"
        }
    ]
}
```

### Emacs

Add to your `.emacs` or `init.el`:

```elisp
(defun kfmtbox-format-region ()
  "Format selected region as flower box"
  (interactive)
  (shell-command-on-region (region-beginning) (region-end) 
    (format "kfmtbox --filename %s" (buffer-file-name)) 
    nil t))

(global-set-key (kbd "C-c f b") 'kfmtbox-format-region)
```

## 🧪 Development

### Prerequisites

- Rust 1.70+ (edition 2021)
- Cargo

### Building

```bash
# Clone the repository
git clone https://github.com/yourusername/kfmtbox.git
cd kfmtbox

# Build in debug mode
cargo build

# Build optimized release
cargo build --release

# Run tests
cargo test

# Check code quality
cargo clippy

# Format code
cargo fmt
```

### Project Structure

```
kfmtbox/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── lib.rs            # Library exports
│   ├── cli.rs            # Command line argument parsing
│   ├── box_renderer.rs   # Flower box rendering logic
│   ├── text_wrapper.rs   # Text wrapping and balancing
│   ├── comment_detector.rs # File type and comment style detection
│   └── config.rs         # YAML configuration handling
├── Cargo.toml            # Project dependencies and metadata
├── README.md             # This file
├── LICENSE               # MIT License
└── .github/              # GitHub workflows and templates
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Quick Contribution Steps

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Run the test suite (`cargo test`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to your branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## 📋 Roadmap

- [ ] **Block comment support** for languages like C/C++ (`/* */`)
- [ ] **Theming system** with different border styles
- [ ] **VSCode extension** for seamless integration
- [ ] **JetBrains plugin** for IntelliJ, PyCharm, etc.
- [ ] **Auto-detection** of file type from stdin context
- [ ] **Configurable padding** and border characters
- [ ] **Multi-line input** with automatic paragraph detection

## 🐛 Bug Reports & Feature Requests

Found a bug or have an idea? We'd love to hear from you!

- 🐛 [Report a bug](https://github.com/yourusername/kfmtbox/issues/new?template=bug_report.md)
- 💡 [Request a feature](https://github.com/yourusername/kfmtbox/issues/new?template=feature_request.md)
- 💬 [Start a discussion](https://github.com/yourusername/kfmtbox/discussions)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by the need for beautiful, consistent comment formatting
- Built with ❤️ using [Rust](https://www.rust-lang.org/)
- Uses [clap](https://github.com/clap-rs/clap) for CLI parsing
- Configuration powered by [serde_yaml](https://github.com/dtolnay/serde-yaml)

---

**Made with ❤️ by the kfmtbox team**

[⭐ Star this repo](https://github.com/yourusername/kfmtbox) if you find it useful!
