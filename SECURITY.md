# Security Policy

## Supported Versions

We release patches for security vulnerabilities in the following versions:

| Version | Supported          |
|---------|--------------------|
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

## Reporting a Vulnerability

We take security bugs seriously. We appreciate your efforts to responsibly disclose your findings, and will make every effort to acknowledge your contributions.

### How to Report a Security Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to: [security@example.com](mailto:security@example.com)

You should receive a response within 48 hours. If for some reason you do not, please follow up via email to ensure we received your original message.

### What to Include

Please include the following information in your report:

- Type of issue (e.g. buffer overflow, SQL injection, cross-site scripting, etc.)
- Full paths of source file(s) related to the manifestation of the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it

### What to Expect

After you submit a report, we will:

1. Confirm receipt of your vulnerability report within 48 hours
2. Provide regular updates on our progress
3. Credit you in our security advisory (unless you prefer to remain anonymous)

### Security Best Practices

When using kfmtbox, please follow these security best practices:

- **Keep kfmtbox updated** to the latest version
- **Validate input** before passing it to kfmtbox
- **Use trusted sources** for configuration files
- **Review configuration files** before using them
- **Run with minimal privileges** when possible

### Security Considerations

kfmtbox is designed with security in mind:

- **No network access** - kfmtbox operates entirely locally
- **Input validation** - All input is validated and sanitized
- **Memory safety** - Written in Rust for memory safety guarantees
- **Dependency auditing** - Regular security audits of dependencies
- **Minimal attack surface** - Simple, focused functionality

### Known Security Issues

Currently, there are no known security vulnerabilities in kfmtbox.

### Security Updates

Security updates will be released as soon as possible after a vulnerability is confirmed and a fix is available. We will:

- Release a new version with the security fix
- Publish a security advisory
- Update this document if needed

### Contact

For security-related questions or concerns, please contact us at:
- Email: [security@example.com](mailto:security@example.com)
- GitHub Security Advisories: [Create a private vulnerability report](https://github.com/yourusername/kfmtbox/security/advisories/new)

Thank you for helping keep kfmtbox and our users safe!
