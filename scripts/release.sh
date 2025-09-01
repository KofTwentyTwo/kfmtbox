#!/bin/bash

# kfmtbox release script
# Usage: ./scripts/release.sh <version>
# Example: ./scripts/release.sh 0.1.0

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if version is provided
if [ $# -eq 0 ]; then
    echo -e "${RED}Error: Version number required${NC}"
    echo "Usage: $0 <version>"
    echo "Example: $0 0.1.0"
    exit 1
fi

VERSION=$1
TAG="v${VERSION}"

echo -e "${BLUE}Releasing kfmtbox version ${VERSION}${NC}"

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo -e "${RED}Error: Not in a git repository${NC}"
    exit 1
fi

# Check if working directory is clean
if ! git diff-index --quiet HEAD --; then
    echo -e "${RED}Error: Working directory is not clean${NC}"
    echo "Please commit or stash your changes before releasing"
    exit 1
fi

# Check if tag already exists
if git rev-parse "$TAG" >/dev/null 2>&1; then
    echo -e "${RED}Error: Tag $TAG already exists${NC}"
    exit 1
fi

# Update version in Cargo.toml
echo -e "${YELLOW}Updating version in Cargo.toml...${NC}"
sed -i.bak "s/^version = \".*\"/version = \"${VERSION}\"/" Cargo.toml
rm Cargo.toml.bak

# Update version in Homebrew formula
echo -e "${YELLOW}Updating version in Homebrew formula...${NC}"
sed -i.bak "s/url \"https:\/\/github\.com\/koftwentytwo\/kfmtbox\/archive\/v.*\.tar\.gz\"/url \"https:\/\/github\.com\/koftwentytwo\/kfmtbox\/archive\/v${VERSION}\.tar\.gz\"/" Formula/kfmtbox.rb
rm Formula/kfmtbox.rb.bak

# Update CHANGELOG.md (if it exists)
if [ -f "CHANGELOG.md" ]; then
    echo -e "${YELLOW}Please update CHANGELOG.md with the new version${NC}"
    echo "Press Enter when done..."
    read -r
fi

# Run tests
echo -e "${YELLOW}Running tests...${NC}"
cargo test

# Run clippy
echo -e "${YELLOW}Running clippy...${NC}"
cargo clippy --all-targets --all-features

# Check formatting
echo -e "${YELLOW}Checking formatting...${NC}"
cargo fmt --check

# Build release
echo -e "${YELLOW}Building release...${NC}"
cargo build --release

# Commit changes
echo -e "${YELLOW}Committing changes...${NC}"
git add Cargo.toml Formula/kfmtbox.rb CHANGELOG.md
git commit -m "Release version ${VERSION}"

# Create and push tag
echo -e "${YELLOW}Creating and pushing tag...${NC}"
git tag -a "$TAG" -m "Release version ${VERSION}"
git push origin main
git push origin "$TAG"

echo -e "${GREEN}Release ${VERSION} created successfully!${NC}"
echo -e "${BLUE}Next steps:${NC}"
echo "1. Wait for GitHub Actions to build and create the release"
echo "2. Update package managers (Homebrew, AUR, etc.)"
echo "3. Announce the release on social media/forums"

# Optional: Open GitHub releases page
if command -v open >/dev/null 2>&1; then
    echo -e "${YELLOW}Opening GitHub releases page...${NC}"
    open "https://github.com/koftwentytwo/kfmtbox/releases"
elif command -v xdg-open >/dev/null 2>&1; then
    echo -e "${YELLOW}Opening GitHub releases page...${NC}"
    xdg-open "https://github.com/koftwentytwo/kfmtbox/releases"
fi