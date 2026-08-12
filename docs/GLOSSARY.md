# ZK Terminology Glossary

## A

**Argument of Knowledge**
A proof system where the prover convinces the verifier that it *knows* a
witness for a statement, not merely that a witness exists.

## B

**BN254 (alt_bn128)**
An elliptic curve used in Groth16 and Plonk. It supports efficient bilinear
pairings and is available as a precompile on Ethereum. Soroban contracts
implement the pairing in WASM.

**Bulletproofs**
A range proof scheme with no trusted setup requirement and logarithmic proof
size. Used to prove that a secret value lies within a range (e.g. 0–2^64).

## C

**Circuit**
A representation of a computation as arithmetic constraints over a finite
field. ZK provers generate proofs that a prover's private inputs satisfy the
circuit constraints.

**Commitment**
A cryptographic binding of a value that can be *revealed* later. Commitments
are hiding (the value is secret) and binding (cannot be changed after commit).

## G

**Groth16**
A zkSNARK construction by Jens Groth (2016) producing the smallest proof of
any current scheme: three elliptic-curve points (~128 bytes). Requires a
per-circuit trusted setup ceremony.

## M

**Merkle Tree**
A binary hash tree where each node is the hash of its two children. The root
commits to all leaves. Membership proofs reveal a path from a leaf to the
root (O(log n) hashes).

**Miller Loop**
An intermediate step in pairing computation on elliptic curves, producing an
element in an extension field for use in the final exponentiation.

## N

**Nullifier**
A unique value derived from a private secret that can be published to mark
a proof as "used" without revealing the secret. Prevents double-spend in
privacy protocols.

## P

**Pairing**
A bilinear map e: G1 × G2 → GT on elliptic curves. Used in Groth16 and Plonk
to check polynomial identities without revealing witnesses.

**Plonk**
A universal zkSNARK with a per-scheme (not per-circuit) trusted setup.
Supports custom gates and lookup arguments (UltraPlonk).

**Public Inputs**
The part of a circuit's inputs that are revealed to the verifier. Private
inputs (witnesses) remain hidden.

## R

**R1CS (Rank-1 Constraint System)**
A matrix-based representation of arithmetic circuits used by Groth16.
Circom compiles circuits to R1CS.

## S

**SNARK**
Succinct Non-interactive ARgument of Knowledge. "Succinct" means the proof is
small and fast to verify relative to the computation proved.

**STARK**
Scalable Transparent ARgument of Knowledge. No trusted setup. Larger proof
size than SNARKs but post-quantum secure.

## T

**Trusted Setup**
A ceremony that generates public parameters for a zkSNARK scheme. If the
ceremony is compromised, fake proofs may be generated. "Transparent" schemes
(STARKs, Plonk with KZG-universal) require minimal or no trust.

## V

**Verification Key (VK)**
The public parameters derived from a trusted setup (or structured reference
string) that allow anyone to verify proofs for a specific circuit.

## W

**Witness**
The private inputs to a circuit. The prover knows the witness; the verifier
does not. ZK proofs convince the verifier that a valid witness exists without
revealing it.

## Z

**ZKP (Zero-Knowledge Proof)**
A cryptographic protocol where a prover demonstrates knowledge of a secret
without revealing any information about the secret itself.
