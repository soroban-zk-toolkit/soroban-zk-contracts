# Groth16 in Soroban

## What is Groth16?

Groth16 (Jens Groth, 2016) is a succinct non-interactive argument of knowledge
(zkSNARK) with the smallest proof size of any current zkSNARK construction:
just **three elliptic-curve points** (≈ 128 bytes on bn254).

It relies on a bilinear pairing check over two groups on the BN254 (alt_bn128)
elliptic curve:

```
e(A, B) == e(alpha, beta) · e(sum_i x_i·IC[i], gamma) · e(C, delta)
```

where `A`, `B`, `C` are proof elements and `IC` is the instance-specific
commitment from the verification key.

## Proof Structure (bn254)

| Element | Encoding | Size |
|---|---|---|
| `A` | G1 compressed point | 32 bytes |
| `B` | G2 compressed point | 64 bytes |
| `C` | G1 compressed point | 32 bytes |
| **Total** | | **128 bytes** |

## Verification Key Structure

| Element | Description |
|---|---|
| `alpha_g1` | Generator times alpha (trusted setup) |
| `beta_g2` | Generator times beta (trusted setup) |
| `gamma_g2` | Generator times gamma (trusted setup) |
| `delta_g2` | Generator times delta (trusted setup) |
| `IC[]` | Instance commitments — one per public input + 1 |

## Mapping to Soroban

### Challenge: no native pairing

Soroban WASM contracts do not have a host-side pairing precompile (unlike
Ethereum's EIP-197). The pairing check must be implemented in pure Rust WASM.

### Strategy

1. Deserialise the proof and VK from `ProofBytes` / `VerificationKey`.
2. Run the Miller loop over the four pairings.
3. Run the final exponentiation.
4. Compare the result with the identity element.
5. Emit `proof_verified` or `proof_rejected` event.

### CPU budget

A single Groth16 pairing check with the `ark-bn254` library compiles to
approximately 50–80 M WASM instructions. Soroban allows up to 100 M
instructions per transaction, so the verifier fits in a single transaction
for up to ~8 public inputs.

## Trusted Setup

Groth16 requires a per-circuit trusted setup ceremony. Use snarkjs:

```bash
snarkjs groth16 setup circuit.r1cs pot12_final.ptau circuit_0000.zkey
snarkjs zkey contribute circuit_0000.zkey circuit_0001.zkey --name="Contributor"
snarkjs zkey export verificationkey circuit_0001.zkey verification_key.json
```

Convert the JSON verification key to bytes for `register_vk`.

## References

- [Groth16 paper](https://eprint.iacr.org/2016/260)
- [snarkjs](https://github.com/iden3/snarkjs)
- [ark-bn254](https://github.com/arkworks-rs/curves)
