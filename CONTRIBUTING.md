# Contributing to kfmtbox

Thank you for your interest in contributing to kfmtbox! This document provides guidelines and information for contributors.

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+ (edition 2021)
- Cargo
- Git

### Development Setup

1. **Fork and clone the repository**
   ```bash
   git clone https://github.com/yourusername/kfmtbox.git
   cd kfmtbox
   ```

2. **Build the project**
   ```bash
   cargo build
   ```

3. **Run tests**
   ```bash
   cargo test
   ```

4. **Check code quality**
   ```bash
   cargo clippy
   cargo fmt
   ```

## 🎯 How to Contribute

### Reporting Bugs

Before creating a bug report, please check existing issues to avoid duplicates. When creating a bug report, include:

- **Clear description** of the bug
- **Steps to reproduce** the issue
- **Expected behavior** vs actual behavior
- **Environment details** (OS, Rust version, etc.)
- **Sample input** that triggers the bug (if applicable)

Use the [bug report template](.github/ISSUE_TEMPLATE/bug_report.md) when creating issues.

### Suggesting Features

We welcome feature suggestions! When proposing a new feature:

- **Describe the problem** it solves
- **Explain the proposed solution**
- **Consider alternatives** you've thought about
- **Provide examples** of how it would work

Use the [feature request template](.github/ISSUE_TEMPLATE/feature_request.md) when creating issues.

### Code Contributions

#### Development Workflow

1. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**
   - Write clean, well-documented code
   - Follow Rust naming conventions
   - Add tests for new functionality
   - Update documentation as needed

3. **Test your changes**
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```

4. **Commit your changes**
   ```bash
   git add .
   git commit -m "Add: brief description of changes"
   ```

5. **Push and create a Pull Request**
   ```bash
   git push origin feature/your-feature-name
   ```

#### Code Style Guidelines

- **Formatting**: Use `cargo fmt` to format code
- **Linting**: Address all `cargo clippy` warnings
- **Documentation**: Document public APIs with doc comments
- **Tests**: Write unit tests for new functionality
- **Naming**: Follow Rust naming conventions (snake_case for functions/variables, PascalCase for types)

#### Commit Message Format

Use clear, descriptive commit messages:

```
Add: support for block comments in C/C++ files
Fix: text wrapping edge case with Unicode characters
Update: improve error messages for invalid file types
Docs: add examples for VSCode integration
Test: add unit tests for comment detection logic
```

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run integration tests
cargo test --test integration_tests
```

### Writing Tests

- **Unit tests**: Place in the same file as the code being tested
- **Integration tests**: Place in `tests/` directory
- **Test coverage**: Aim for good test coverage of new functionality
- **Edge cases**: Test edge cases and error conditions

### Test Examples

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment_detection() {
        let detector = CommentDetector::new();
        let style = detector.detect_from_filetype("rs");
        assert_eq!(style.prefix, "//");
    }

    #[test]
    fn test_text_wrapping() {
        let wrapper = TextWrapper::new(20);
        let result = wrapper.wrap_text("This is a very long line");
        assert!(result.len() > 1);
        for line in &result {
            assert!(line.len() <= 20);
        }
    }
}
```

## 📁 Project Structure

```
kfmtbox/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library exports
│   ├── cli.rs               # Command line parsing
│   ├── box_renderer.rs      # Flower box rendering
│   ├── text_wrapper.rs      # Text wrapping logic
│   ├── comment_detector.rs  # File type detection
│   └── config.rs            # Configuration handling
├── tests/                   # Integration tests
├── .github/                 # GitHub workflows and templates
├── Cargo.toml              # Project configuration
├── README.md               # Project documentation
├── CONTRIBUTING.md         # This file
└── LICENSE                 # MIT License
```

## 🔍 Code Review Process

### Pull Request Guidelines

- **Clear description** of changes and motivation
- **Link to related issues** if applicable
- **Screenshots or examples** for UI changes
- **Update documentation** as needed
- **Ensure all tests pass**

### Review Checklist

- [ ] Code follows project style guidelines
- [ ] Tests are included and passing
- [ ] Documentation is updated
- [ ] No breaking changes (or clearly documented)
- [ ] Performance implications considered
- [ ] Security implications reviewed

## 🐛 Common Issues and Solutions

### Build Issues

**Problem**: `cargo build` fails with dependency errors
**Solution**: Run `cargo update` to update dependencies

**Problem**: Tests fail on specific platforms
**Solution**: Check if the issue is platform-specific and add appropriate test conditions

### Code Issues

**Problem**: Clippy warnings
**Solution**: Address all warnings or add `#[allow(clippy::warning_name)]` with justification

**Problem**: Formatting issues
**Solution**: Run `cargo fmt` to auto-format code

## 📚 Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Clap Documentation](https://docs.rs/clap/)
- [Serde Documentation](https://serde.rs/)

## 💬 Getting Help

- **GitHub Discussions**: For questions and general discussion
- **GitHub Issues**: For bug reports and feature requests
- **Pull Request Comments**: For code review discussions

## 🎉 Recognition

Contributors will be recognized in:
- README.md contributors section
- Release notes for significant contributions
- GitHub contributor graph

Thank you for contributing to kfmtbox! 🚀