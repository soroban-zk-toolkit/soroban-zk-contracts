# Admin Transfer — Two-Step Ownership Confirmation

## Overview

This document describes the two-step admin ownership transfer pattern for the Soroban ZK contracts. A single-step transfer is dangerous because a typo or compromised call could permanently lock out the rightful owner. Two-step confirmation requires the **new** admin to explicitly accept the role before the handover is finalised.

## Motivation

- **Prevent accidental lockout**: if the current admin sends to a wrong address, the handover is not committed until acceptance.
- **Prevent phishing handovers**: even if an attacker tricks the current admin into initiating a transfer, the attacker's address must sign the acceptance — confirming they control the private key.
- **Audit trail**: two on-chain transactions provide a clear record of intent and acceptance.

## Storage Keys

| Key | Type | Description |
|-----|------|-------------|
| `Admin` | `Address` | Current admin address |
| `PendingAdmin` | `Address` | Nominated next admin (not yet confirmed) |

## Functions

### 1. `transfer_admin` — Current admin nominates a new admin

```rust
pub fn transfer_admin(env: Env, current_admin: Address, new_admin: Address) {
    current_admin.require_auth();
    require_admin(&env, &current_admin);

    env.storage().instance().set(&Symbol::new(&env, "PendingAdmin"), &new_admin);

    env.events().publish(
        (Symbol::new(&env, "admin_transfer_initiated"),),
        (current_admin, new_admin),
    );
}
```

### 2. `accept_admin` — Pending admin confirms and takes ownership

```rust
pub fn accept_admin(env: Env, new_admin: Address) {
    new_admin.require_auth();

    let pending: Address = env.storage().instance()
        .get(&Symbol::new(&env, "PendingAdmin"))
        .expect("no pending admin");

    if pending != new_admin {
        panic!("caller is not the pending admin");
    }

    // Commit the transfer
    env.storage().instance().set(&Symbol::new(&env, "Admin"), &new_admin);
    env.storage().instance().remove(&Symbol::new(&env, "PendingAdmin"));

    env.events().publish(
        (Symbol::new(&env, "admin_transfer_completed"),),
        new_admin,
    );
}
```

### 3. `cancel_admin_transfer` — Current admin revokes nomination

```rust
pub fn cancel_admin_transfer(env: Env, current_admin: Address) {
    current_admin.require_auth();
    require_admin(&env, &current_admin);

    env.storage().instance().remove(&Symbol::new(&env, "PendingAdmin"));

    env.events().publish(
        (Symbol::new(&env, "admin_transfer_cancelled"),),
        current_admin,
    );
}
```

## Events

| Event topic | Data | Meaning |
|-------------|------|---------|
| `admin_transfer_initiated` | `(current_admin, new_admin)` | Nomination recorded |
| `admin_transfer_completed` | `new_admin` | Ownership transferred |
| `admin_transfer_cancelled` | `current_admin` | Nomination revoked |

## Operational Flow

```
Current Admin                  New Admin
     │                              │
     │── transfer_admin(new) ──────▶│  (PendingAdmin stored)
     │                              │
     │                              │── accept_admin() ──▶ Admin updated
     │                              │
     │  (or)                        │
     │── cancel_admin_transfer() ──▶│  (PendingAdmin cleared)
```

## Security Considerations

- There is **no expiry** on a pending nomination by default. Consider adding a ledger-based deadline if your threat model requires it.
- Only one pending admin can exist at a time; a new `transfer_admin` call overwrites the previous nomination.
- Combine with a multisig wallet as the admin address for maximum security.
- Never set the zero address or a burn address as `PendingAdmin`.

## References

- OpenZeppelin Ownable2Step (EVM reference): https://docs.openzeppelin.com/contracts/4.x/api/access#Ownable2Step
- Soroban instance storage: https://developers.stellar.org/docs/smart-contracts/storage/instance
