# Style Guide for kfmtbox

This project follows strict coding standards based on C++ Allman brace style and formatting preferences.

## IDE Integration Strategy

With RustIDE using nightly Rust, we now have **full Allman brace style support**:

1. **IDE Formatting**: Uses Allman brace style (opening braces on new lines)
2. **Automatic Enforcement**: Allman style applied automatically by rustfmt
3. **Consistent Indentation**: 3-space indentation (matches your C++ style)
4. **Quality Checks**: Clippy enforces code quality and style consistency

## Brace Style: Allman Style (Manual Enforcement)

**Opening braces `{` should be on a new line after control structures when manually formatting.**

### ✅ Correct (Allman Style):
```rust
if condition
{
    // code here
}

for item in items
{
    // code here
}

while condition
{
    // code here
}

match value
{
    Some(x) => {
        // code here
    }
    None => {
        // code here
    }
}

impl MyStruct
{
    fn method(&self)
    {
        // code here
    }
}
```

### ❌ Incorrect (K&R Style):
```rust
if condition {
    // code here
}

for item in items {
    // code here
}

impl MyStruct {
    fn method(&self) {
        // code here
    }
}
```

## Formatting Configuration

### For IDE Integration (Recommended)
- **Indentation**: 3 spaces (matches your C++ style)
- **Line length**: 100 characters
- **Brace style**: Allman style (opening braces on new lines)
- **Automatic enforcement**: Applied by nightly rustfmt

### For Manual Allman Style (Optional)
If you want to manually format with Allman brace style:

```bash
# Use the nightly formatting script
./scripts/format.sh

# Or manually use nightly rustfmt
rustup run nightly cargo fmt --config rustfmt-nightly.toml
```

### IDE Setup
Your RustIDE with nightly Rust will automatically:
- Use 3-space indentation
- Enforce 100-character line limits
- Format with Allman brace style (opening braces on new lines)
- Run clippy for quality checks
- Apply consistent import grouping

## Enforcement

### IDE Integration (Primary)
Your IDE will automatically:
1. **Format code** with 3-space indentation and 100-char lines
2. **Run clippy** for quality and style warnings
3. **Check formatting** on save
4. **Provide suggestions** for improvements

### Manual Enforcement (Secondary)
For Allman brace style specifically:
1. **Code review** - All PRs should follow Allman style where possible
2. **Documentation** - This style guide serves as reference
3. **Optional script** - Use `./scripts/format.sh` for Allman formatting

### Automated Checks
```bash
# Check formatting (IDE compatible)
cargo fmt --check

# Run clippy for quality warnings
cargo clippy --all-targets --all-features

# Run tests
cargo test

# Optional: Format with Allman style
./scripts/format.sh
```

## Configuration Files

- `rustfmt.toml` - Stable Rust formatting (3-space indentation, 100-char lines)
- `rustfmt-nightly.toml` - Nightly Rust formatting (includes Allman brace style)
- `clippy.toml` - Linting rules for code quality
- `Cargo.toml` - Project-level linting configuration

## Examples

### Function Definitions
```rust
fn my_function(param1: String, param2: usize) -> Result<String, Error>
{
    if param1.is_empty()
    {
        return Err(Error::InvalidInput);
    }
    
    for i in 0..param2
    {
        // process each item
    }
    
    Ok("success".to_string())
}
```

### Struct and Impl Blocks
```rust
pub struct MyStruct
{
    field1: String,
    field2: usize,
}

impl MyStruct
{
    pub fn new(field1: String, field2: usize) -> Self
    {
        Self { field1, field2 }
    }
    
    pub fn process(&self) -> Result<(), Error>
    {
        if self.field1.is_empty()
        {
            return Err(Error::InvalidState);
        }
        
        Ok(())
    }
}
```

### Match Statements
```rust
match result
{
    Ok(value) => {
        println!("Success: {}", value);
    }
    Err(error) => {
        eprintln!("Error: {}", error);
    }
}
```

## Integration with CI/CD

The GitHub Actions workflow will:
1. Check formatting with `cargo fmt --check`
2. Run clippy with strict rules
3. Run tests
4. Build for multiple platforms

All code must pass these checks before merging.

## Migration Guide

If you have existing code that doesn't follow Allman style:

1. **Manual conversion**: Update braces to be on new lines
2. **Use nightly formatter**: `rustup run nightly cargo fmt --config rustfmt-nightly.toml`
3. **Verify with clippy**: `cargo clippy --all-targets --all-features`

## Benefits

- **Consistency**: Matches your C++ coding style
- **Readability**: Clear visual separation of control structures
- **Maintainability**: Easier to read and modify code
- **Professional**: Industry-standard formatting for many C++ projects