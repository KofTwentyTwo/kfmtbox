#!/bin/bash

# Format script for kfmtbox with Allman brace style
# This script uses nightly Rust to format code with Allman brace style

set -e

echo "🔧 Formatting code with Allman brace style..."

# Check if nightly Rust is installed
if ! /Users/james.maes/.cargo/bin/rustup show | grep -q "nightly"; then
    echo "📦 Installing nightly Rust..."
    /Users/james.maes/.cargo/bin/rustup install nightly
fi

# Use nightly rustfmt with Allman brace style configuration
echo "🎨 Running rustfmt with Allman brace style..."
/Users/james.maes/.cargo/bin/rustup run nightly cargo fmt

echo "✅ Formatting complete!"
echo ""
echo "📋 Style applied:"
echo "   - 3-space indentation"
echo "   - 100-character line limit"
echo "   - Allman brace style (opening braces on new lines)"
echo "   - Consistent spacing and formatting"
echo ""
echo "💡 Your RustIDE should now format with Allman brace style automatically!"