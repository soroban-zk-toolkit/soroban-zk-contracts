# Security Policy

## Supported Versions

| Version | Supported |
|---|---|
| `main` branch | Yes |
| `develop` branch | Best-effort |
| < 0.1.0 | No |

## Reporting a Vulnerability

**Please do not open a public GitHub issue for security vulnerabilities.**

### How to Report

1. Email **security@soroban-zk-toolkit.dev** with the subject line:
   `[SECURITY] soroban-zk-contracts — <brief description>`

2. Include in your report:
   - A clear description of the vulnerability
   - Steps to reproduce or a proof-of-concept
   - The potential impact (e.g. fund loss, proof forgery, DoS)
   - Any suggested mitigations

3. Encrypt your email with our PGP key if the issue is highly sensitive
   (key available on request).

### Response Timeline

| Stage | Target |
|---|---|
| Initial acknowledgement | 48 hours |
| Triage and severity assessment | 5 business days |
| Fix or mitigation patch | 14–30 days (severity-dependent) |
| Public disclosure | After fix is deployed |

## Scope

In scope:
- All Soroban contract code in `contracts/`
- Proof verification logic (false accept / false reject vulnerabilities)
- Storage layout vulnerabilities (nullifier bypass, VK manipulation)
- Access control issues (unauthorised admin functions)

Out of scope:
- Third-party libraries (report upstream)
- Issues requiring physical access to a validator node
- Theoretical attacks with no practical exploit path

## Bug Bounty

A bug bounty programme is planned for `v0.2.0`. Details will be published in
this file when the programme launches.

## Acknowledgements

We thank all security researchers who responsibly disclose vulnerabilities.
Accepted reports will be credited in the release notes unless the reporter
requests anonymity.
