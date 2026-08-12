# Roadmap

## v0.1.0 — Core Verifiers (target: 2026-10-01)

- [ ] Groth16 proof verifier (bn254)
- [ ] Plonk proof verifier
- [ ] Merkle membership proof verifier
- [ ] Nullifier registry (double-spend prevention)
- [ ] Range proof verifier (private amounts)
- [ ] Owner-only verification key registration
- [ ] Structured events on verification accept/reject
- [ ] Storage TTL extension on all entry points
- [ ] Full rustdoc coverage
- [ ] Integration test suite (Groth16)
- [ ] Contract size and CPU benchmarks
- [ ] WASM build and deployment guide

## v0.2.0 — Audit & Mainnet (target: 2027-01-01)

- [ ] Third-party security audit
- [ ] Address all audit findings
- [ ] TypeScript SDK helper for proof submission
- [ ] Mainnet deployment
- [ ] Bug bounty programme launch
- [ ] Multi-circuit registry (multiple VKs per scheme)
- [ ] Batched proof verification (multiple proofs per tx)
- [ ] On-chain PLONK verifier optimisation
- [ ] Recursion support (proof of proofs)

## v0.3.0 — Ecosystem (target: 2027-07-01)

- [ ] STARK verifier (FRI-based)
- [ ] Nova folding scheme support
- [ ] Cross-contract verification API (IZKVerifier interface)
- [ ] Example privacy-preserving DeFi contract using the verifier
- [ ] Developer workshops and tutorials

## Ideas Backlog

- Verifier-as-a-service (off-chain relay that submits verified proofs)
- ZK identity / credential verification
- Private NFT ownership proof
- Confidential voting contract using range proofs

---

> This roadmap is a living document. Priorities may shift based on community
> feedback and ecosystem developments. Open an issue to suggest new items.
