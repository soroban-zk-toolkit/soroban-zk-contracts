# Frequently Asked Questions

## General

### What is a zero-knowledge proof?

A zero-knowledge proof (ZKP) lets one party (the prover) convince another
(the verifier) that a statement is true without revealing any information
beyond the truth of the statement itself. For example, a user can prove they
know a secret password without revealing the password.

### Why run ZK verification on-chain?

On-chain verification is trustless — anyone can audit the contract, and the
result is part of the blockchain's immutable record. This removes the need for
a trusted intermediary to check proofs.

### Which ZK schemes are supported?

| Scheme | Status |
|---|---|
| Groth16 (bn254) | Planned — v0.1.0 |
| Plonk / UltraPlonk | Planned — v0.1.0 |
| Merkle membership | Planned — v0.1.0 |
| Bulletproofs (range) | Planned — v0.1.0 |
| STARK / FRI | Planned — v0.3.0 |
| Nova folding | Planned — v0.3.0 |

## Technical

### Does Soroban have a pairing precompile?

No. Unlike Ethereum (EIP-197), Soroban has no host-level pairing precompile.
The pairing check runs in pure Rust WASM inside the contract. This is
feasible because Soroban allows up to 100 M CPU instructions per transaction.

### How large is the contract WASM?

With size-optimised release profile and `wasm-opt -Oz`, the contract binary
is expected to be under 200 KB. Benchmarks will be published in v0.1.0.

### How do I generate proofs?

Use an off-chain proving library such as:
- [snarkjs](https://github.com/iden3/snarkjs) (JavaScript/TypeScript)
- [circom](https://github.com/iden3/circom) (circuit compiler)
- [ark-groth16](https://github.com/arkworks-rs/groth16) (Rust)

See the [Integration Guide](INTEGRATION.md) for a full example.

### What happens if I send a malformed proof?

The contract returns `VerifierError::InvalidProof` (code 1). It never panics
on bad input.

### Can the verification key be updated?

Yes. The admin can call `register_vk` with a new verification key for the
same `key_id` to overwrite it. Plan key rotation carefully — old proofs
generated for the previous key will no longer verify.

## Security

### Is the contract audited?

Not yet. A formal third-party audit is planned for v0.2.0. Do not use this
contract in production before the audit is complete.

### How do I report a vulnerability?

See [SECURITY.md](../SECURITY.md) for the responsible disclosure policy.
