# Quick Setup Guide for Package Distribution

This guide helps you quickly set up `kfmtbox` for distribution across multiple package managers.

## 🚀 Quick Start (5 minutes)

### 1. First Release Setup

```bash
# 1. Set up GitHub secrets (in your repository settings)
# Go to: Settings > Secrets and variables > Actions
# Add these secrets:
# - CARGO_REGISTRY_TOKEN (from crates.io account)
# - HOMEBREW_CORE_TOKEN (GitHub token with repo access)

# 2. Create your first release
./scripts/release.sh 0.1.0

# 3. Wait for GitHub Actions to complete
# 4. Check the release at: https://github.com/koftwentytwo/kfmtbox/releases
```

### 2. Package Manager Setup

#### Homebrew (Personal Tap)
```bash
# Create a new repository: koftwentytwo/homebrew-kfmtbox
# Copy Formula/kfmtbox.rb to your tap repository
# Users can then install with:
brew tap koftwentytwo/kfmtbox
brew install kfmtbox
```

#### Cargo (crates.io)
```bash
# 1. Create account at https://crates.io
# 2. Get API token from account settings
# 3. Add token to GitHub secrets as CARGO_REGISTRY_TOKEN
# 4. GitHub Actions will automatically publish on release
```

#### Arch Linux (AUR)
```bash
# 1. Create AUR account at https://aur.archlinux.org
# 2. Clone the AUR repository:
git clone ssh://aur@aur.archlinux.org/kfmtbox.git
cd kfmtbox

# 3. Create PKGBUILD (see PACKAGING.md for template)
# 4. Create .SRCINFO: makepkg --printsrcinfo > .SRCINFO
# 5. Commit and push
```

## 📋 Pre-Release Checklist

- [ ] Update `Cargo.toml` version
- [ ] Update `CHANGELOG.md`
- [ ] Run `cargo test` and `cargo clippy`
- [ ] Test installation scripts locally
- [ ] Verify all documentation links work

## 🔧 Troubleshooting

### Common Issues

1. **GitHub Actions fails**: Check secrets are set correctly
2. **Cargo publish fails**: Verify API token and package name
3. **Homebrew formula fails**: Check SHA256 hash in formula
4. **Installation scripts fail**: Test on clean system

### Getting Help

- Check the full [PACKAGING.md](PACKAGING.md) for detailed instructions
- Review [GitHub Actions logs](https://github.com/koftwentytwo/kfmtbox/actions)
- Test installation scripts on different platforms

## 🎯 Success Metrics

Track your distribution success:

- **Downloads**: GitHub release statistics
- **Cargo installs**: `cargo install --list | grep kfmtbox`
- **Homebrew installs**: `brew info kfmtbox`
- **User feedback**: GitHub issues and discussions

## 📈 Next Steps

After successful initial release:

1. **Monitor usage**: Check download statistics
2. **Gather feedback**: Respond to issues and feature requests
3. **Expand distribution**: Add more package managers
4. **Improve documentation**: Update README with user examples
5. **Community building**: Share on social media and forums

## 🚀 Pro Tips

- **Start small**: Begin with GitHub releases and Cargo
- **Automate everything**: Use GitHub Actions for all releases
- **Test thoroughly**: Verify installation on multiple platforms
- **Document everything**: Keep PACKAGING.md updated
- **Engage community**: Respond to issues and feature requests quickly