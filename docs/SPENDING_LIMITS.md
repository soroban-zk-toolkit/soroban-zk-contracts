# Spending Limit Per Epoch — Verifier Registry Abuse Prevention

## Overview

This document describes the epoch-based spending limit design for the Soroban ZK verifier registry contract. The goal is to prevent abuse by capping how many verifications (or how much fees/gas) any single caller can consume within a defined epoch window.

## Motivation

Without per-epoch limits, a malicious or buggy caller could:
- Flood the verifier registry with cheap spam verifications
- Exhaust contract storage or compute budgets
- Denial-of-service legitimate callers by monopolising verification slots

## Design

### Epoch Definition

An epoch is a fixed number of Stellar ledgers. A recommended default is **17,280 ledgers ≈ 24 hours** (Stellar closes ~1 ledger per 5 seconds).

```rust
const EPOCH_LEDGERS: u32 = 17_280; // ~24 h
```

The current epoch for any ledger `L` is:

```
epoch = L / EPOCH_LEDGERS
```

### Storage Keys

| Key | Type | Description |
|-----|------|-------------|
| `SpendingLimit` | `u32` | Global cap on verifications per epoch per caller |
| `EpochUsage(Address, epoch)` | `u32` | How many verifications `Address` has used in `epoch` |

### Enforcement Logic

Before executing a proof verification, the contract checks:

```rust
fn check_spending_limit(env: &Env, caller: &Address) {
    let current_epoch = env.ledger().sequence() / EPOCH_LEDGERS;
    let key = (caller.clone(), current_epoch);
    let used: u32 = env.storage().temporary().get(&key).unwrap_or(0);
    let limit: u32 = env.storage().instance().get(&Symbol::new(env, "SpendingLimit"))
        .unwrap_or(100); // default 100/epoch
    if used >= limit {
        panic!("spending limit exceeded for this epoch");
    }
    env.storage().temporary().set(&key, &(used + 1));
}
```

Temporary storage is ideal here because usage data only needs to survive for one epoch and can be automatically purged afterwards.

### Admin Configuration

The contract admin can update the global spending limit:

```rust
fn set_spending_limit(env: &Env, admin: Address, new_limit: u32) {
    admin.require_auth();
    require_admin(env, &admin);
    env.storage().instance().set(&Symbol::new(env, "SpendingLimit"), &new_limit);
}
```

### TTL Management

Temporary entries should have their TTL set to at least one epoch duration so they are not purged prematurely:

```rust
env.storage().temporary().extend_ttl(&key, EPOCH_LEDGERS, EPOCH_LEDGERS);
```

## Security Considerations

- The limit applies **per caller address per epoch**, not globally, to avoid penalising legitimate users.
- The admin should tune the limit based on observed usage patterns.
- If the limit is set to `0`, no calls are allowed — useful during a maintenance epoch.
- Combine with the pause mechanism (`PAUSE_MECHANISM.md`) for full emergency control.

## Future Enhancements

- **Tiered limits**: different caps for allow-listed addresses (e.g., trusted dApps).
- **Dynamic limits**: auto-adjust based on on-chain congestion signals.
- **Fee burns**: instead of hard caps, charge an increasing fee per call within an epoch.

## References

- Soroban temporary storage docs: https://developers.stellar.org/docs/smart-contracts/storage/temporary
- Stellar ledger close time: https://developers.stellar.org/docs/learn/fundamentals/stellar-consensus-protocol
