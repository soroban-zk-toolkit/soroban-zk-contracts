# Contract Upgrade Pattern — Soroban Instance Storage Approach

## Overview

This document describes the contract upgrade pattern used in the Soroban ZK contracts. Soroban supports in-place WASM upgrades via the `update_current_contract_wasm` host function, using the contract's instance storage to coordinate the upgrade safely.

## Why Soroban Upgrades Are Different

Unlike EVM proxy patterns, Soroban contracts:
- Have no separate proxy/implementation split — the contract ID stays the same
- Are upgraded by uploading a new WASM blob to the ledger and calling `update_current_contract_wasm`
- Do NOT automatically migrate storage — any storage layout changes require explicit migration logic

## Upgrade Entry Point

```rust
pub fn upgrade(env: Env, admin: Address, new_wasm_hash: BytesN<32>) {
    // 1. Auth check — only admin can trigger upgrade
    admin.require_auth();
    require_admin(&env, &admin);

    // 2. Pause the contract to prevent state changes during upgrade
    require_not_paused(&env); // upgrade itself must be called while unpaused by the admin

    // 3. Perform the WASM swap — takes effect at the START of the NEXT transaction
    env.deployer().update_current_contract_wasm(new_wasm_hash);

    // 4. Emit upgrade event for off-chain monitoring
    env.events().publish(
        (Symbol::new(&env, "contract_upgraded"), admin.clone()),
        new_wasm_hash,
    );
}
```

> **Important**: `update_current_contract_wasm` does **not** immediately replace the running WASM. The new WASM is used starting from the next invocation of the contract after the upgrade transaction is finalized.

## Storage Migration

If the new WASM version changes the storage schema, a migration entry point must be called **once** after the upgrade:

```rust
pub fn migrate(env: Env, admin: Address) {
    admin.require_auth();
    require_admin(&env, &admin);

    let migrated: bool = env.storage().instance()
        .get(&Symbol::new(&env, "V2Migrated"))
        .unwrap_or(false);
    if migrated {
        panic!("already migrated");
    }

    // Example: rename a storage key from V1 to V2 schema
    if let Some(old_val) = env.storage().instance().get::<_, OldType>(&Symbol::new(&env, "OldKey")) {
        let new_val = transform(old_val);
        env.storage().instance().set(&Symbol::new(&env, "NewKey"), &new_val);
        env.storage().instance().remove(&Symbol::new(&env, "OldKey"));
    }

    env.storage().instance().set(&Symbol::new(&env, "V2Migrated"), &true);

    env.events().publish(
        (Symbol::new(&env, "storage_migrated"),),
        (),
    );
}
```

### Migration Safety Rules

- The `migrate` function must be **idempotent** — calling it twice must be a no-op or safe error.
- Mark migration as complete in instance storage (`V2Migrated` flag above).
- Test migration on a testnet fork with a copy of the mainnet state snapshot before upgrading production.
- If migration fails mid-way, the contract should remain functional on old storage schema until a corrected migration is deployed.

## Upgrade Checklist

- [ ] New WASM uploaded to the ledger: `stellar contract upload --wasm new_version.wasm --network mainnet`
- [ ] WASM hash verified against the expected build hash
- [ ] Contract paused (optional — recommended for complex migrations)
- [ ] `upgrade(admin, new_wasm_hash)` called
- [ ] `migrate(admin)` called if storage schema changed
- [ ] Contract unpaused
- [ ] Smoke tests passed (see `MAINNET_CHECKLIST.md`)

## Instance Storage and Upgrade TTL

After an upgrade, the contract's instance storage TTL must be extended to keep the contract live:

```rust
env.storage().instance().extend_ttl(MIN_TTL, MAX_TTL);
```

The WASM entry on the ledger also needs its TTL extended separately — this is typically handled by the deployer account's `extend_footprint_ttl` operation.

## Events

| Event topic | Data | Meaning |
|-------------|------|---------|
| `contract_upgraded` | `new_wasm_hash: BytesN<32>` | WASM replaced by admin |
| `storage_migrated` | `()` | Storage migration completed |

## References

- Soroban upgrade docs: https://developers.stellar.org/docs/smart-contracts/guides/upgrades
- `update_current_contract_wasm`: https://docs.rs/soroban-sdk/latest/soroban_sdk/struct.Deployer.html
- TTL management: https://developers.stellar.org/docs/smart-contracts/storage/state-archival
