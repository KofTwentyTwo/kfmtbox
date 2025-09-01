# Package Distribution Guide

This document explains how to distribute `kfmtbox` through various package managers and platforms.

## 📦 Package Managers

### 1. Homebrew (macOS/Linux)

#### Option A: Personal Tap (Recommended for initial release)
1. Create a personal tap repository: `koftwentytwo/homebrew-kfmtbox`
2. Add the formula from `Formula/kfmtbox.rb` to your tap
3. Users can install with:
   ```bash
   brew tap koftwentytwo/kfmtbox
   brew install kfmtbox
   ```

#### Option B: Homebrew Core (For wider distribution)
1. Submit a PR to [homebrew-core](https://github.com/Homebrew/homebrew-core)
2. Follow the [Homebrew formula guidelines](https://docs.brew.sh/Formula-Cookbook)
3. Requirements:
   - At least 20 stars on GitHub
   - At least 3 months old
   - Stable API
   - No dependencies on non-free software

### 2. Cargo (Rust)

#### Prerequisites
1. Create account on [crates.io](https://crates.io)
2. Get API token from account settings
3. Add token to GitHub secrets as `CARGO_REGISTRY_TOKEN`

#### Publishing
```bash
# Login to crates.io
cargo login

# Publish (done automatically via GitHub Actions)
cargo publish
```

### 3. Arch Linux (AUR)

Create a PKGBUILD file:

```bash
# Create AUR package
git clone ssh://aur@aur.archlinux.org/kfmtbox.git
cd kfmtbox

# Add PKGBUILD (see below)
git add PKGBUILD .SRCINFO
git commit -m "Initial release"
git push
```

**PKGBUILD template:**
```bash
pkgname=kfmtbox
pkgver=0.1.0
pkgrel=1
pkgdesc="A Rust-based command-line utility that wraps text in flower box style comments"
arch=('x86_64')
url="https://github.com/koftwentytwo/kfmtbox"
license=('MIT')
depends=('glibc')
makedepends=('cargo' 'git')
source=("$pkgname-$pkgver.tar.gz::https://github.com/koftwentytwo/kfmtbox/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$pkgname-$pkgver"
    cargo build --release --locked
}

package() {
    cd "$pkgname-$pkgver"
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    install -Dm644 "README.md" "$pkgdir/usr/share/doc/$pkgname/README.md"
    install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
```

### 4. Debian/Ubuntu (PPA)

#### Prerequisites
1. Create Launchpad account
2. Set up PPA
3. Install packaging tools: `sudo apt install devscripts debhelper`

#### Creating Debian Package
```bash
# Create package structure
mkdir kfmtbox-0.1.0
cd kfmtbox-0.1.0

# Copy source files
cp -r ../src .
cp ../Cargo.toml ../Cargo.lock .

# Create debian directory
mkdir debian

# Add control file, rules, etc.
# (See Debian packaging documentation)
```

### 5. Windows (Chocolatey)

#### Prerequisites
1. Create Chocolatey account
2. Install Chocolatey package validator

#### Package Structure
```
kfmtbox/
├── kfmtbox.nuspec
├── tools/
│   ├── chocolateyinstall.ps1
│   └── chocolateyuninstall.ps1
└── legal/
    └── LICENSE.txt
```

**kfmtbox.nuspec:**
```xml
<?xml version="1.0" encoding="utf-8"?>
<package xmlns="http://schemas.microsoft.com/packaging/2015/06/nuspec.xsd">
  <metadata>
    <id>kfmtbox</id>
    <version>0.1.0</version>
    <title>kfmtbox</title>
    <authors>James Maes</authors>
    <description>A Rust-based command-line utility that wraps text in flower box style comments</description>
    <licenseUrl>https://github.com/koftwentytwo/kfmtbox/blob/main/LICENSE</licenseUrl>
    <projectUrl>https://github.com/koftwentytwo/kfmtbox</projectUrl>
    <tags>cli formatting comments text-processing</tags>
  </metadata>
  <files>
    <file src="tools\**" target="tools" />
    <file src="legal\**" target="legal" />
  </files>
</package>
```

### 6. Snap (Ubuntu/Snap Store)

#### Prerequisites
1. Create Snapcraft account
2. Install snapcraft: `sudo snap install snapcraft --classic`

#### snapcraft.yaml
```yaml
name: kfmtbox
version: '0.1.0'
summary: A Rust-based command-line utility that wraps text in flower box style comments
description: |
  Transform your plain text into beautiful, decorative comment boxes that automatically
  adapt to your programming language's comment syntax.

grade: stable
confinement: strict

parts:
  kfmtbox:
    plugin: rust
    source: .
    build-packages:
      - gcc
      - pkg-config
      - libssl-dev

apps:
  kfmtbox:
    command: kfmtbox
    plugs:
      - home
```

## 🚀 Release Process

### 1. Automated Releases (GitHub Actions)

The project includes automated release workflows that:

1. **Build binaries** for multiple platforms:
   - Linux (x86_64, musl)
   - macOS (x86_64, ARM64)
   - Windows (x86_64, MSVC and GNU)

2. **Publish to crates.io** (when `CARGO_REGISTRY_TOKEN` is set)

3. **Create GitHub releases** with downloadable binaries

### 2. Manual Release Steps

1. **Update version** in `Cargo.toml`
2. **Create git tag**: `git tag v0.1.0`
3. **Push tag**: `git push origin v0.1.0`
4. **GitHub Actions** will automatically:
   - Build and test
   - Create release
   - Upload binaries
   - Publish to crates.io (if configured)

### 3. Package Manager Updates

After each release, update package managers:

1. **Homebrew**: Update formula with new version and SHA
2. **AUR**: Update PKGBUILD with new version
3. **Snap**: Update snapcraft.yaml version
4. **Chocolatey**: Update nuspec version

## 📋 Checklist for New Release

- [ ] Update version in `Cargo.toml`
- [ ] Update `CHANGELOG.md`
- [ ] Create and push git tag
- [ ] Verify GitHub Actions release
- [ ] Update Homebrew formula (if using personal tap)
- [ ] Update AUR package
- [ ] Update other package managers
- [ ] Test installation from all package managers
- [ ] Update documentation

## 🔧 Troubleshooting

### Common Issues

1. **Homebrew formula fails**: Check SHA256 hash and URL
2. **Cargo publish fails**: Verify API token and package name availability
3. **AUR package rejected**: Check PKGBUILD syntax and dependencies
4. **Snap build fails**: Verify snapcraft.yaml syntax

### Getting Help

- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [Cargo Publishing Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [AUR User Guidelines](https://wiki.archlinux.org/title/AUR_user_guidelines)
- [Snapcraft Documentation](https://snapcraft.io/docs)

## 📊 Distribution Statistics

Track your package downloads and usage:

- **Crates.io**: [crates.io/crates/kfmtbox](https://crates.io/crates/kfmtbox)
- **Homebrew**: `brew info kfmtbox`
- **GitHub**: Release download statistics
- **AUR**: AUR package page statistics