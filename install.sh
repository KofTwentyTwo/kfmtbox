#!/bin/bash

# kfmtbox installation script
# Supports Linux, macOS, and Windows (via WSL)

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPO="koftwentytwo/kfmtbox"
BINARY_NAME="kfmtbox"
INSTALL_DIR="${HOME}/.local/bin"

# Detect OS and architecture
detect_platform() {
    local os arch
    
    case "$(uname -s)" in
        Linux*)     os="linux" ;;
        Darwin*)    os="macos" ;;
        CYGWIN*|MINGW*|MSYS*) os="windows" ;;
        *)          echo -e "${RED}Unsupported operating system${NC}"; exit 1 ;;
    esac
    
    case "$(uname -m)" in
        x86_64|amd64) arch="x86_64" ;;
        arm64|aarch64) arch="aarch64" ;;
        *) echo -e "${RED}Unsupported architecture${NC}"; exit 1 ;;
    esac
    
    echo "${os}-${arch}"
}

# Get latest release version
get_latest_version() {
    curl -s "https://api.github.com/repos/${REPO}/releases/latest" | \
    grep '"tag_name":' | \
    sed -E 's/.*"([^"]+)".*/\1/'
}

# Download and install binary
install_binary() {
    local platform=$1
    local version=$2
    local asset_name
    
    # Determine asset name based on platform
    case "$platform" in
        "linux-x86_64")
            asset_name="kfmtbox-linux-x86_64"
            ;;
        "macos-x86_64")
            asset_name="kfmtbox-macos-x86_64"
            ;;
        "macos-aarch64")
            asset_name="kfmtbox-macos-aarch64"
            ;;
        "windows-x86_64")
            asset_name="kfmtbox-windows-x86_64.exe"
            ;;
        *)
            echo -e "${RED}Unsupported platform: ${platform}${NC}"
            exit 1
            ;;
    esac
    
    local download_url="https://github.com/${REPO}/releases/download/${version}/${asset_name}"
    local temp_file="/tmp/${asset_name}"
    
    echo -e "${BLUE}Downloading ${asset_name}...${NC}"
    curl -L -o "$temp_file" "$download_url"
    
    # Create install directory if it doesn't exist
    mkdir -p "$INSTALL_DIR"
    
    # Make binary executable and move to install directory
    chmod +x "$temp_file"
    mv "$temp_file" "${INSTALL_DIR}/${BINARY_NAME}"
    
    echo -e "${GREEN}Installed ${BINARY_NAME} to ${INSTALL_DIR}/${BINARY_NAME}${NC}"
}

# Add to PATH if needed
add_to_path() {
    local shell_rc
    
    case "$SHELL" in
        */zsh) shell_rc="${HOME}/.zshrc" ;;
        */bash) shell_rc="${HOME}/.bashrc" ;;
        *) shell_rc="${HOME}/.profile" ;;
    esac
    
    if ! echo "$PATH" | grep -q "${INSTALL_DIR}"; then
        echo -e "${YELLOW}Adding ${INSTALL_DIR} to PATH in ${shell_rc}${NC}"
        echo "export PATH=\"${INSTALL_DIR}:\$PATH\"" >> "$shell_rc"
        echo -e "${GREEN}Please restart your shell or run: source ${shell_rc}${NC}"
    fi
}

# Main installation function
main() {
    echo -e "${BLUE}Installing kfmtbox...${NC}"
    
    # Detect platform
    local platform
    platform=$(detect_platform)
    echo -e "${BLUE}Detected platform: ${platform}${NC}"
    
    # Get latest version
    local version
    version=$(get_latest_version)
    echo -e "${BLUE}Latest version: ${version}${NC}"
    
    # Install binary
    install_binary "$platform" "$version"
    
    # Add to PATH
    add_to_path
    
    echo -e "${GREEN}Installation complete!${NC}"
    echo -e "${BLUE}Try running: kfmtbox --help${NC}"
}

# Check if running as root
if [[ $EUID -eq 0 ]]; then
   echo -e "${RED}This script should not be run as root${NC}"
   exit 1
fi

# Check dependencies
if ! command -v curl &> /dev/null; then
    echo -e "${RED}curl is required but not installed${NC}"
    exit 1
fi

# Run main function
main "$@"