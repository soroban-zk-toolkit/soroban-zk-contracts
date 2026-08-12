# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Workspace `Cargo.toml` for the `zk-verifier` contract crate
- `src/lib.rs` — entry points: `verify_groth16`, `verify_plonk`, `verify_merkle`, `register_vk`
- `src/types.rs` — `ProofBytes`, `PublicInputs`, `VerificationKey`, `VerifierError`
- `src/events.rs` — `proof_verified` and `proof_rejected` event helpers
- `src/storage.rs` — typed storage keys and TTL-aware read/write helpers
- Architecture, Groth16, Deployment, and Integration documentation
- `SECURITY.md` — responsible disclosure policy
- `ROADMAP.md` — project roadmap
- `CHANGELOG.md` — this file
- Build and deploy helper scripts
- Example verification shell script
- GitHub issue templates and PR template
- FAQ and ZK glossary

## [0.1.0-alpha] - 2026-08-12

### Added
- Initial repository structure
- README, CONTRIBUTING, CODEOWNERS
- GitHub Actions CI skeleton
- `.env.example`

[Unreleased]: https://github.com/soroban-zk-toolkit/soroban-zk-contracts/compare/v0.1.0-alpha...HEAD
[0.1.0-alpha]: https://github.com/soroban-zk-toolkit/soroban-zk-contracts/releases/tag/v0.1.0-alpha
