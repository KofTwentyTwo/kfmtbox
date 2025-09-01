# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of kfmtbox
- Support for flower box comment formatting
- File type detection from filename and explicit filetype
- Text wrapping with balanced line lengths
- Existing comment detection and reformatting
- YAML configuration support for custom comment styles
- Support for 25+ programming languages
- CLI interface with comprehensive options
- Unit tests for core functionality
- GitHub Actions CI/CD pipeline
- Comprehensive documentation

### Features
- Beautiful Unicode box drawing characters (╭╮╰╯│─)
- Smart text wrapping with configurable max width
- Automatic comment style detection
- Configuration file support
- Editor integration examples
- Cross-platform support (Linux, macOS, Windows)

## [0.1.0] - 2024-09-01

### Added
- Initial release
- Core flower box rendering functionality
- File type detection system
- Text wrapping and balancing
- YAML configuration support
- CLI argument parsing with clap
- Comprehensive test suite
- Documentation and examples

### Supported File Types
- Rust (.rs) → //
- Python (.py) → #
- JavaScript (.js) → //
- TypeScript (.ts) → //
- Go (.go) → //
- Java (.java) → //
- C/C++ (.c, .cpp, .h, .hpp) → //
- C# (.cs) → //
- PHP (.php) → //
- Ruby (.rb) → #
- Shell (.sh, .bash, .zsh, .fish) → #
- Vim (.vim) → "
- Lua (.lua) → --
- SQL (.sql) → --
- YAML (.yaml, .yml) → #
- TOML (.toml) → #
- INI (.ini, .cfg) → ;
- Dockerfile → #
- Makefile → #
- CMake (CMakeLists.txt) → #

### CLI Options
- `-f, --filename` - Input filename for file type detection
- `-t, --filetype` - File type for comment style detection
- `-w, --max-width` - Maximum line width (default: 120)
- `-h, --help` - Print help information
- `-V, --version` - Print version information

### Configuration
- YAML configuration file support
- Custom comment style definitions
- Platform-specific config locations
- Default fallback comment styles

### Testing
- Unit tests for all core modules
- Integration tests for CLI functionality
- Text wrapping and balancing tests
- Comment detection and extraction tests
- Cross-platform compatibility testing

### Documentation
- Comprehensive README with examples
- Contributing guidelines
- Code of conduct
- Security policy
- API documentation
- Editor integration examples

---

## Version Numbering

This project uses [Semantic Versioning](https://semver.org/):

- **MAJOR** version when you make incompatible API changes
- **MINOR** version when you add functionality in a backwards compatible manner
- **PATCH** version when you make backwards compatible bug fixes

## Types of Changes

- **Added** for new features
- **Changed** for changes in existing functionality
- **Deprecated** for soon-to-be removed features
- **Removed** for now removed features
- **Fixed** for any bug fixes
- **Security** for vulnerability fixes