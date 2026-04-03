# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability in Civiqo, please report it responsibly.

**Do NOT open a public GitHub issue for security vulnerabilities.**

Instead, please email: **security@civiqo.com**

Include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

## Response Timeline

- **Acknowledgment**: within 48 hours
- **Assessment**: within 1 week
- **Fix**: as soon as possible, depending on severity

## Supported Versions

| Version | Supported |
|---------|-----------|
| latest  | Yes       |

## Security Practices

- Passwords are hashed with Argon2
- JWT tokens use HS256 with minimum 32-byte secrets
- Template auto-escaping is enabled to prevent XSS
- SQL queries use parameterized statements via SQLx
- Session cookies use SameSite=Lax
