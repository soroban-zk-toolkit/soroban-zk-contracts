# Batch Proof Verification — Gas-Efficient Entry Point

## Overview

This document describes the batch proof verification design for the Soroban ZK contracts. A single entry point that verifies multiple proofs in one transaction dramatically reduces per-proof overhead for high-throughput applications.

## Motivation

Verifying proofs one-at-a-time incurs repeated:
- Contract invocation overhead (auth checks, storage reads)
- Event emission costs
- Stellar base fee per operation

Batch verification amortises these costs across N proofs, enabling use cases like:
- ZK rollup settlement submitting dozens of user proofs per batch
- Recurring oracle attestations
- Privacy-preserving voting where all ballots are settled at epoch close

## Design

### Entry Point Signature

```rust
#[contracttype]
pub struct ProofEntry {
    pub vk_id: Symbol,
    pub proof: Bytes,
    pub public_inputs: Bytes,
}

#[contracttype]
pub struct VerificationResult {
    pub index: u32,
    pub success: bool,
    pub reason: Option<Symbol>, // None on success; Some("invalid_proof" | "revoked_key" | "unknown_key") on failure
}

pub fn verify_proofs_batch(
    env: Env,
    caller: Address,
    proofs: Vec<ProofEntry>,
) -> Vec<VerificationResult> {
    require_not_paused(&env);
    caller.require_auth();
    check_spending_limit(&env, &caller); // counts as N uses or 1 batch use depending on policy

    let mut results = Vec::new(&env);
    for (index, entry) in proofs.iter().enumerate() {
        let result = try_verify_single(&env, &entry);
        results.push_back(VerificationResult {
            index: index as u32,
            success: result.is_ok(),
            reason: result.err(),
        });
    }
    results
}
```

### Error Handling Strategy

The batch entry point **never panics** on individual proof failure. Instead it:
1. Records the failure in the result vector.
2. Continues processing remaining proofs.
3. Returns the full result set to the caller.

This allows partial success — a caller can identify which proofs failed without resubmitting the entire batch.

### Spending Limit Integration

Two policy options:

| Policy | Description |
|--------|-------------|
| **Per-proof counting** | Each proof in the batch counts against the epoch limit individually. Fairer, prevents batch abuse. |
| **Per-call counting** | The entire batch counts as one call. Simpler, favours batchers. |

Recommended default: **per-proof counting**. Update `check_spending_limit` to accept a `count: u32` parameter:

```rust
fn check_spending_limit(env: &Env, caller: &Address, count: u32) {
    // ... add `count` to used, reject if used + count > limit
}
```

### Events

One event per proof outcome keeps off-chain indexers in sync:

```rust
// emitted inside try_verify_single on success
env.events().publish(
    (Symbol::new(&env, "proof_verified"), caller.clone()),
    (entry.vk_id.clone(), index),
);

// emitted on failure
env.events().publish(
    (Symbol::new(&env, "proof_rejected"), caller.clone()),
    (entry.vk_id.clone(), index, reason),
);
```

A single `batch_settled` summary event is also emitted after the loop:

```rust
env.events().publish(
    (Symbol::new(&env, "batch_settled"), caller.clone()),
    (total, succeeded, failed),
);
```

## Performance Considerations

- Soroban imposes a **CPU instruction limit** per transaction. Large batches may hit this limit before all proofs are processed. Callers should benchmark batch sizes and choose a safe maximum (typically 10–50 proofs depending on circuit complexity).
- Use `Vec` with a pre-allocated capacity where possible to reduce dynamic allocation overhead.
- Avoid repeated storage reads for the same `vk_id` within a batch — cache the loaded verification key in a local variable.

## Caller-Side Batching Guidance

```typescript
// Example: split 100 proofs into batches of 20
const BATCH_SIZE = 20;
for (let i = 0; i < proofs.length; i += BATCH_SIZE) {
  const batch = proofs.slice(i, i + BATCH_SIZE);
  await contract.call("verify_proofs_batch", caller, batch);
}
```

## References

- Soroban resource limits: https://developers.stellar.org/docs/smart-contracts/resource-limits-fees
- Soroban Vec type: https://docs.rs/soroban-sdk/latest/soroban_sdk/struct.Vec.html
