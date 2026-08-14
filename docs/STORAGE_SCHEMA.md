# Storage Key Schema — Soroban ZK Contracts

## Overview

This document provides a complete reference for all storage keys used in the `soroban-zk-contracts` verifier contract. Understanding the storage layout is essential for:
- Writing upgrade migration logic (see `UPGRADE_PATTERN.md`)
- Building off-chain indexers that decode raw ledger state
- Auditing contract behaviour without running entry points

## Storage Tiers

Soroban provides three storage tiers with different persistence and cost characteristics:

| Tier | Persistence | Use case |
|------|------------|----------|
| **Instance** | Lives as long as the contract instance | Config, admin, flags |
| **Persistent** | Archived after TTL; survives restores | Verification keys, long-lived data |
| **Temporary** | Purged after TTL; cannot be restored | Epoch usage counters, nonces |

---

## Instance Storage Keys

| Key (Symbol) | Value Type | Description |
|-------------|-----------|-------------|
| `Admin` | `Address` | Current contract administrator |
| `PendingAdmin` | `Address` | Nominated admin awaiting confirmation (see `ADMIN_TRANSFER.md`) |
| `Paused` | `bool` | Emergency pause flag (`true` = halted) |
| `SpendingLimit` | `u32` | Maximum verifications per caller per epoch |
| `V2Migrated` | `bool` | Whether v2 storage migration has run |

---

## Persistent Storage Keys

### Verification Key Registry

| Key | Key Type | Value Type | Description |
|-----|----------|-----------|-------------|
| `VkEntry(vk_id)` | `(Symbol("VkEntry"), Symbol)` | `VerificationKeyEntry` | Full VK record indexed by `vk_id` |
| `VkRevoked(vk_id)` | `(Symbol("VkRevoked"), Symbol)` | `bool` | Revocation flag for a VK |

#### `VerificationKeyEntry` struct

```rust
#[contracttype]
pub struct VerificationKeyEntry {
    pub vk_bytes: Bytes,          // raw verification key serialised bytes
    pub vk_hash: BytesN<32>,      // SHA-256 of vk_bytes for integrity checks
    pub registered_at: u32,       // ledger sequence at registration
    pub registered_by: Address,   // admin address that registered this key
}
```

---

## Temporary Storage Keys

| Key | Key Type | Value Type | TTL | Description |
|-----|----------|-----------|-----|-------------|
| `EpochUsage(caller, epoch)` | `(Address, u32)` | `u32` | `EPOCH_LEDGERS` | Number of verifications caller has used in `epoch` |

`epoch` is computed as `ledger_sequence / EPOCH_LEDGERS` where `EPOCH_LEDGERS = 17_280`.

---

## Key Encoding Details

All storage keys are encoded using Soroban's XDR serialisation. When building off-chain queries:

### Single Symbol key (instance storage)
```
SCVal::Symbol("Admin")
```

### Tuple key (persistent storage — VK entry)
```
SCVal::Vec([
  SCVal::Symbol("VkEntry"),
  SCVal::Symbol("<vk_id>"),
])
```

### Tuple key (temporary storage — epoch usage)
```
SCVal::Vec([
  SCVal::Address(<caller>),
  SCVal::U32(<epoch_number>),
])
```

---

## Ledger State Query Example

Using the Stellar RPC `getLedgerEntries` method to read a VK entry:

```bash
VK_ID="my_circuit_v1"
CONTRACT_ID="CXXX..."

# Encode the key as base64 XDR (use stellar-xdr CLI or js-stellar-base)
KEY_XDR=$(stellar contract read --id $CONTRACT_ID --key "VkEntry,$VK_ID" --network mainnet)
echo $KEY_XDR
```

Using the JavaScript SDK:

```typescript
import { xdr, Address } from "@stellar/stellar-sdk";

const key = xdr.ScVal.scvVec([
  xdr.ScVal.scvSymbol("VkEntry"),
  xdr.ScVal.scvSymbol("my_circuit_v1"),
]);
```

---

## TTL Management Reference

| Storage type | Default TTL | Recommended extension |
|-------------|-------------|----------------------|
| Instance | Contract lifetime | Extend after each upgrade |
| Persistent (VK entries) | Configurable | `min_ttl=100_000, max_ttl=6_000_000` (~1 year) |
| Temporary (epoch usage) | `EPOCH_LEDGERS` | Auto-extends on each use |

---

## Notes for Upgraders

- If a new version adds a new instance storage key, no migration is needed — the key simply returns `None` until set.
- If a key is **renamed**, write a migration function to copy the old key value to the new key name and remove the old key.
- If a **value type changes**, write a migration function to re-encode all existing values in the new format.
- Always set the `V2Migrated` (or `VNMigrated`) flag after migration to prevent double-runs.

## References

- Soroban storage docs: https://developers.stellar.org/docs/smart-contracts/storage
- XDR encoding: https://developers.stellar.org/docs/learn/encyclopedia/data-format/xdr
- `stellar contract read` CLI: https://developers.stellar.org/docs/tools/developer-tools/cli/stellar-cli
