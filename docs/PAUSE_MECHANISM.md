# Contract Pause Mechanism — Emergency Halt Pattern for Soroban

## Overview

This document describes the emergency pause mechanism for the Soroban ZK contracts. A pause mechanism allows the contract admin to immediately halt all state-changing operations in response to a discovered vulnerability, exploit, or operational incident.

## Motivation

Zero-knowledge proof verification contracts handle sensitive cryptographic state. In the event of:
- A discovered bug in the verifier logic
- An unexpected upgrade incompatibility
- An active exploit draining contract funds or corrupting state

…the ability to freeze the contract instantly is critical to limiting damage.

## Design

### Pause Flag Storage

The pause state is stored in instance storage so it persists across ledgers and is immediately visible to all callers:

```rust
const PAUSED_KEY: &str = "Paused";

fn is_paused(env: &Env) -> bool {
    env.storage().instance().get::<_, bool>(&Symbol::new(env, PAUSED_KEY))
        .unwrap_or(false)
}
```

### Guard Macro / Helper

Every state-changing entry point begins with a pause check:

```rust
fn require_not_paused(env: &Env) {
    if is_paused(env) {
        panic!("contract is paused");
    }
}
```

Example usage in a verification entry point:

```rust
pub fn verify_proof(env: Env, caller: Address, proof: Bytes) -> bool {
    require_not_paused(&env);
    caller.require_auth();
    // ... verification logic
}
```

Read-only views (e.g. `get_verification_key`) may optionally remain unpaused so monitoring tools can still inspect state.

### Admin Controls

```rust
pub fn pause(env: Env, admin: Address) {
    admin.require_auth();
    require_admin(&env, &admin);
    env.storage().instance().set(&Symbol::new(&env, PAUSED_KEY), &true);
    env.events().publish((Symbol::new(&env, "contract_paused"),), ());
}

pub fn unpause(env: Env, admin: Address) {
    admin.require_auth();
    require_admin(&env, &admin);
    env.storage().instance().set(&Symbol::new(&env, PAUSED_KEY), &false);
    env.events().publish((Symbol::new(&env, "contract_unpaused"),), ());
}
```

### Events

Both pause and unpause emit events so off-chain monitoring systems can react immediately (trigger alerts, halt relayers, etc.).

| Event topic | Data | Meaning |
|-------------|------|---------|
| `contract_paused` | `()` | Contract halted by admin |
| `contract_unpaused` | `()` | Contract resumed by admin |

## Operational Runbook

1. **Detect incident** — monitoring alerts on anomalous verification rate or state change.
2. **Pause contract** — admin calls `pause()` within seconds via a hot-wallet or multisig.
3. **Investigate** — with the contract frozen, analyse on-chain state without further damage.
4. **Deploy fix** — upgrade the contract via the instance storage upgrade pattern (see `UPGRADE_PATTERN.md`).
5. **Unpause** — admin calls `unpause()` after verifying the fix is live.
6. **Post-mortem** — document root cause and adjust spending limits or access controls as needed.

## Security Considerations

- Only the contract admin (or a multisig) should hold the authority to call `pause` / `unpause`.
- Consider a **timelock** on `unpause` to enforce a mandatory review window before resuming.
- Emit events on both actions to ensure full audit trail.
- The pause flag must be checked **before** any auth checks to prevent paused-state bypasses.

## References

- Soroban instance storage: https://developers.stellar.org/docs/smart-contracts/storage/instance
- Soroban events: https://developers.stellar.org/docs/smart-contracts/events
