//! Storage layout and helpers for the ZK verifier contract.
//!
//! All persistent data uses typed storage keys to avoid collisions and make
//! the storage schema easy to audit.

use soroban_sdk::{contracttype, Bytes, Env};

use crate::types::{VerificationKey, VerifierError};

/// Storage key namespace for the verifier contract.
#[contracttype]
#[derive(Clone)]
pub enum StorageKey {
    /// Maps a circuit `key_id` to its serialised `VerificationKey`.
    VerificationKey(Bytes),
    /// Tracks spent nullifiers: key → `true` if spent.
    Nullifier(Bytes),
    /// The admin address allowed to register verification keys.
    Admin,
}

/// Number of ledgers a persistent entry should remain live.
/// Approximately 30 days at a ~5 s ledger close time.
const PERSISTENT_TTL_LEDGERS: u32 = 518_400;

/// Write a verification key to persistent storage.
pub fn write_vk(env: &Env, key_id: Bytes, vk: VerificationKey) {
    let storage_key = StorageKey::VerificationKey(key_id);
    env.storage().persistent().set(&storage_key, &vk);
    env.storage()
        .persistent()
        .extend_ttl(&storage_key, PERSISTENT_TTL_LEDGERS, PERSISTENT_TTL_LEDGERS);
}

/// Read a verification key from persistent storage.
///
/// Returns `Err(VerifierError::UnknownVerificationKey)` if the key is absent.
pub fn read_vk(env: &Env, key_id: Bytes) -> Result<VerificationKey, VerifierError> {
    let storage_key = StorageKey::VerificationKey(key_id);
    env.storage()
        .persistent()
        .get(&storage_key)
        .ok_or(VerifierError::UnknownVerificationKey)
}

/// Mark a nullifier as spent.
pub fn mark_nullifier_spent(env: &Env, nullifier: Bytes) {
    let storage_key = StorageKey::Nullifier(nullifier);
    env.storage().persistent().set(&storage_key, &true);
    env.storage()
        .persistent()
        .extend_ttl(&storage_key, PERSISTENT_TTL_LEDGERS, PERSISTENT_TTL_LEDGERS);
}

/// Check whether a nullifier has already been spent.
pub fn is_nullifier_spent(env: &Env, nullifier: Bytes) -> bool {
    let storage_key = StorageKey::Nullifier(nullifier);
    env.storage()
        .persistent()
        .get(&storage_key)
        .unwrap_or(false)
}
