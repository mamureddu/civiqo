# Security

## Reporting a vulnerability

**Do NOT open a public issue for security vulnerabilities.**

Use [GitHub Security Advisories](https://github.com/mamureddu/civiqo/security/advisories/new) to report privately. I'll respond as soon as I can.

## What's in place

- Passwords hashed with Argon2
- JWT (HS256) with minimum 32-byte secrets
- Template auto-escaping enabled (XSS prevention)
- Parameterized SQL queries via SQLx
- Session cookies with SameSite=Lax
