# Architecture Overview

## System Diagram

```
┌─────────────────────────────────────────────────────┐
│                  dApp / Client                      │
│  (generates proof off-chain with snarkjs / circom)  │
└───────────────────────┬─────────────────────────────┘
                        │  invoke contract entry point
                        ▼
┌─────────────────────────────────────────────────────┐
│           ZK Verifier Soroban Contract              │
│                                                     │
│  ┌─────────────┐  ┌──────────────┐  ┌───────────┐  │
│  │ verify_groth16 │ verify_plonk │  │verify_merkle│ │
│  └──────┬──────┘  └──────┬───────┘  └─────┬─────┘  │
│         │                │                │         │
│         └────────────────┼────────────────┘         │
│                          │                          │
│                   ┌──────▼──────┐                   │
│                   │  Verifier   │                   │
│                   │   Core      │                   │
│                   └──────┬──────┘                   │
│                          │                          │
│           ┌──────────────┼────────────┐             │
│           │              │            │             │
│    ┌──────▼──────┐ ┌─────▼────┐ ┌────▼──────┐      │
│    │  VK Storage │ │ Nullifier│ │  Events   │      │
│    │  (Persistent│ │ Registry │ │  (pub/sub)│      │
│    │   Ledger)   │ │          │ │           │      │
│    └─────────────┘ └──────────┘ └───────────┘      │
└─────────────────────────────────────────────────────┘
```

## Components

### Entry Points

| Function | Purpose |
|---|---|
| `verify_groth16` | Verify a Groth16 zk-SNARK proof (bn254 curve) |
| `verify_plonk` | Verify a Plonk/UltraPlonk proof |
| `verify_merkle` | Verify a Merkle membership proof |
| `register_vk` | Admin-only: store a new verification key |

### Storage

Persistent ledger storage with automatic TTL extension:

- `VerificationKey(key_id)` — maps circuit id → serialised VK
- `Nullifier(hash)` — spent nullifier registry
- `Admin` — authorised admin address

### Events

Every accepted or rejected proof emits a structured event on the Stellar
network, enabling off-chain indexers and dApps to react in real time without
polling contract state.

## Design Principles

1. **Fail loudly** — malformed inputs return typed errors, never panic.
2. **Storage TTL** — all persistent entries have their TTL extended at every
   write to prevent unexpected expiration.
3. **Minimal on-chain logic** — proof generation is always off-chain; the
   contract only runs the deterministic verification check.
4. **Upgradeable keys** — verification keys are stored separately from the
   contract code so circuits can be updated without redeployment.
