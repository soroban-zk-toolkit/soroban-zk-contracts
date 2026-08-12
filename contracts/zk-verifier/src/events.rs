//! Event helpers for the ZK verifier contract.
//!
//! Every successful or failed proof verification emits a structured Soroban
//! event so that off-chain indexers and dApps can react without polling state.

use soroban_sdk::{symbol_short, Bytes, Env};

/// Topic used for all events emitted by this contract.
const TOPIC: &str = "zk_verifier";

/// Emit an event signalling that a proof was accepted.
///
/// # Arguments
/// * `env`        - The contract execution environment.
/// * `scheme`     - Short label for the proof scheme ("groth16", "plonk", "merkle").
/// * `circuit_id` - Identifier of the circuit/verification key used.
pub fn proof_verified(env: &Env, scheme: &str, circuit_id: Bytes) {
    let topic = (symbol_short!(TOPIC), symbol_short!("verified"));
    let data = (soroban_sdk::String::from_str(env, scheme), circuit_id);
    env.events().publish(topic, data);
}

/// Emit an event signalling that a proof was rejected.
///
/// # Arguments
/// * `env`        - The contract execution environment.
/// * `scheme`     - Short label for the proof scheme ("groth16", "plonk", "merkle").
/// * `error_code` - The numeric error code from `VerifierError`.
pub fn proof_rejected(env: &Env, scheme: &str, error_code: u32) {
    let topic = (symbol_short!(TOPIC), symbol_short!("rejected"));
    let data = (soroban_sdk::String::from_str(env, scheme), error_code);
    env.events().publish(topic, data);
}
