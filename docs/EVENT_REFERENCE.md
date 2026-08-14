# Event Reference — Verifier Contract Events

## Overview

This document lists all events emitted by the Soroban ZK verifier contracts. Events are emitted via `env.events().publish(topics, data)` and are queryable by off-chain indexers, monitoring systems, and dApp frontends.

Soroban events have:
- **Topics**: a tuple of `Val`s used for filtering (max 4 elements, first is conventionally the event name `Symbol`)
- **Data**: arbitrary `Val` payload

## Verification Key Registry Events

### `vk_registered`

Emitted when a new verification key is registered in the registry.

| Field | Type | Description |
|-------|------|-------------|
| Topic[0] | `Symbol("vk_registered")` | Event identifier |
| Topic[1] | `Address` | Admin who registered the key |
| Data.vk_id | `Symbol` | Identifier for this verification key |
| Data.vk_hash | `BytesN<32>` | SHA-256 hash of the raw verification key bytes |
| Data.registered_at | `u32` | Ledger sequence number at registration time |

```rust
env.events().publish(
    (Symbol::new(&env, "vk_registered"), admin.clone()),
    (vk_id.clone(), vk_hash, env.ledger().sequence()),
);
```

### `vk_revoked`

Emitted when a verification key is revoked and can no longer be used for proof verification.

| Field | Type | Description |
|-------|------|-------------|
| Topic[0] | `Symbol("vk_revoked")` | Event identifier |
| Topic[1] | `Address` | Admin who revoked the key |
| Data.vk_id | `Symbol` | Identifier of the revoked key |
| Data.revoked_at | `u32` | Ledger sequence number at revocation time |

```rust
env.events().publish(
    (Symbol::new(&env, "vk_revoked"), admin.clone()),
    (vk_id.clone(), env.ledger().sequence()),
);
```

### `vk_updated`

Emitted when an existing verification key entry is replaced (e.g. due to a trusted setup ceremony update).

| Field | Type | Description |
|-------|------|-------------|
| Topic[0] | `Symbol("vk_updated")` | Event identifier |
| Topic[1] | `Address` | Admin who performed the update |
| Data.vk_id | `Symbol` | Identifier of the updated key |
| Data.old_vk_hash | `BytesN<32>` | Hash of the previous key |
| Data.new_vk_hash | `BytesN<32>` | Hash of the replacement key |
| Data.updated_at | `u32` | Ledger sequence number |

## Proof Verification Events

### `proof_verified`

Emitted on a successful proof verification.

| Field | Type | Description |
|-------|------|-------------|
| Topic[0] | `Symbol("proof_verified")` | Event identifier |
| Topic[1] | `Address` | Caller who submitted the proof |
| Data.vk_id | `Symbol` | Verification key used |
| Data.public_inputs_hash | `BytesN<32>` | Hash of the public inputs |
| Data.ledger | `u32` | Ledger sequence at verification time |

### `proof_rejected`

Emitted when a proof fails verification (invalid proof, wrong key, or revoked key).

| Field | Type | Description |
|-------|------|-------------|
| Topic[0] | `Symbol("proof_rejected")` | Event identifier |
| Topic[1] | `Address` | Caller who submitted the proof |
| Data.vk_id | `Symbol` | Verification key used |
| Data.reason | `Symbol` | One of: `invalid_proof`, `revoked_key`, `unknown_key` |
| Data.ledger | `u32` | Ledger sequence at rejection time |

## Admin Events

### `admin_transfer_initiated`

See `ADMIN_TRANSFER.md` for the full two-step transfer event series.

### `contract_paused` / `contract_unpaused`

See `PAUSE_MECHANISM.md` for the pause/unpause event definitions.

## Off-chain Indexing

To subscribe to these events using the Stellar Horizon / RPC API:

```bash
# Stream all events from the contract
curl "https://rpc-mainnet.stellar.org" \
  -X POST -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getEvents",
    "params": {
      "startLedger": 1000000,
      "filters": [
        {
          "type": "contract",
          "contractIds": ["CONTRACT_ADDRESS_HERE"],
          "topics": [["vk_registered"]]
        }
      ]
    }
  }'
```

## Notes

- All `Symbol` values are limited to 32 UTF-8 characters in Soroban.
- `BytesN<32>` hashes use big-endian encoding.
- Ledger sequence numbers increase monotonically; use them as logical timestamps.
- Events are not stored in contract storage — they live in the transaction metadata and must be indexed externally.
