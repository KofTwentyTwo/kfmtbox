# kfmtbox installation script for Windows PowerShell
# Run with: powershell -ExecutionPolicy Bypass -File install.ps1

param(
    [string]$Version = "latest"
)

# Configuration
$Repo = "koftwentytwo/kfmtbox"
$BinaryName = "kfmtbox.exe"
$InstallDir = "$env:USERPROFILE\AppData\Local\kfmtbox\bin"

# Colors for output
function Write-ColorOutput($ForegroundColor) {
    $fc = $host.UI.RawUI.ForegroundColor
    $host.UI.RawUI.ForegroundColor = $ForegroundColor
    if ($args) {
        Write-Output $args
    } else {
        $input | Write-Output
    }
    $host.UI.RawUI.ForegroundColor = $fc
}

# Detect architecture
function Get-Architecture {
    if ([Environment]::Is64BitOperatingSystem) {
        return "x86_64"
    } else {
        return "x86"
    }
}

# Get latest release version
function Get-LatestVersion {
    try {
        $response = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"
        return $response.tag_name
    } catch {
        Write-ColorOutput Red "Failed to get latest version: $_"
        exit 1
    }
}

# Download and install binary
function Install-Binary {
    param(
        [string]$Version,
        [string]$Architecture
    )
    
    $assetName = "kfmtbox-windows-$Architecture.exe"
    $downloadUrl = "https://github.com/$Repo/releases/download/$Version/$assetName"
    $tempFile = "$env:TEMP\$assetName"
    
    Write-ColorOutput Blue "Downloading $assetName..."
    try {
        Invoke-WebRequest -Uri $downloadUrl -OutFile $tempFile
    } catch {
        Write-ColorOutput Red "Failed to download binary: $_"
        exit 1
    }
    
    # Create install directory
    if (!(Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    }
    
    # Move binary to install directory
    Move-Item -Path $tempFile -Destination "$InstallDir\$BinaryName" -Force
    
    Write-ColorOutput Green "Installed $BinaryName to $InstallDir\$BinaryName"
}

# Add to PATH
function Add-ToPath {
    $currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    
    if ($currentPath -notlike "*$InstallDir*") {
        Write-ColorOutput Yellow "Adding $InstallDir to PATH..."
        $newPath = "$currentPath;$InstallDir"
        [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
        Write-ColorOutput Green "Added to PATH. Please restart your terminal."
    } else {
        Write-ColorOutput Green "Already in PATH"
    }
}

# Main installation function
function Main {
    Write-ColorOutput Blue "Installing kfmtbox..."
    
    # Detect architecture
    $arch = Get-Architecture
    Write-ColorOutput Blue "Detected architecture: $arch"
    
    # Get version
    if ($Version -eq "latest") {
        $version = Get-LatestVersion
    } else {
        $version = $Version
    }
    Write-ColorOutput Blue "Version: $version"
    
    # Install binary
    Install-Binary -Version $version -Architecture $arch
    
    # Add to PATH
    Add-ToPath
    
    Write-ColorOutput Green "Installation complete!"
    Write-ColorOutput Blue "Try running: kfmtbox --help"
}

# Check if running as administrator
if (([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Write-ColorOutput Red "This script should not be run as administrator"
    exit 1
}

# Check dependencies
try {
    Invoke-WebRequest -Uri "https://github.com" -UseBasicParsing | Out-Null
} catch {
    Write-ColorOutput Red "Internet connection required for installation"
    exit 1
}

# Run main function
Main