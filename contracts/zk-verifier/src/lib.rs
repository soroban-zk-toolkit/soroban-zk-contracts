//! # ZK Verifier Contract
//!
//! A Soroban smart contract that exposes entry points for verifying
//! zero-knowledge proofs on-chain: Groth16, Plonk, and Merkle membership.

#![no_std]

mod events;
mod storage;
mod types;

use soroban_sdk::{contract, contractimpl, Env};
use types::{ProofBytes, PublicInputs, VerificationKey, VerifierError};

/// The ZK Verifier contract.
#[contract]
pub struct ZkVerifierContract;

#[contractimpl]
impl ZkVerifierContract {
    /// Verify a Groth16 zk-SNARK proof.
    ///
    /// # Arguments
    /// * `proof`  - The serialised Groth16 proof bytes.
    /// * `inputs` - The public inputs that accompany the proof.
    /// * `vk`     - The verification key to use.
    ///
    /// # Returns
    /// `Ok(true)` if the proof is valid, `Err(VerifierError)` otherwise.
    pub fn verify_groth16(
        env: Env,
        proof: ProofBytes,
        inputs: PublicInputs,
        vk: VerificationKey,
    ) -> Result<bool, VerifierError> {
        let _ = (&env, &proof, &inputs, &vk);
        // TODO: implement Groth16 pairing-check verification
        Err(VerifierError::NotImplemented)
    }

    /// Verify a Plonk zero-knowledge proof.
    ///
    /// # Arguments
    /// * `proof`  - The serialised Plonk proof bytes.
    /// * `inputs` - The public inputs that accompany the proof.
    /// * `vk`     - The verification key to use.
    ///
    /// # Returns
    /// `Ok(true)` if the proof is valid, `Err(VerifierError)` otherwise.
    pub fn verify_plonk(
        env: Env,
        proof: ProofBytes,
        inputs: PublicInputs,
        vk: VerificationKey,
    ) -> Result<bool, VerifierError> {
        let _ = (&env, &proof, &inputs, &vk);
        // TODO: implement Plonk / UltraPlonk verification
        Err(VerifierError::NotImplemented)
    }

    /// Verify a Merkle tree membership proof.
    ///
    /// # Arguments
    /// * `proof` - The serialised Merkle path proof bytes.
    /// * `root`  - The expected Merkle root (as a public input).
    /// * `leaf`  - The leaf value being proved (as a public input).
    ///
    /// # Returns
    /// `Ok(true)` if the leaf is a member of the tree, `Err(VerifierError)` otherwise.
    pub fn verify_merkle(
        env: Env,
        proof: ProofBytes,
        root: PublicInputs,
        leaf: PublicInputs,
    ) -> Result<bool, VerifierError> {
        let _ = (&env, &proof, &root, &leaf);
        // TODO: implement Merkle path verification
        Err(VerifierError::NotImplemented)
    }

    /// Register a verification key in contract storage.
    ///
    /// Only the contract admin may call this entry point.
    ///
    /// # Arguments
    /// * `key_id` - A unique identifier for the verification key.
    /// * `vk`     - The verification key bytes to store.
    pub fn register_vk(
        env: Env,
        key_id: soroban_sdk::Bytes,
        vk: VerificationKey,
    ) -> Result<(), VerifierError> {
        let _ = (&env, &key_id, &vk);
        // TODO: access control + storage write
        Err(VerifierError::Unauthorized)
    }
}
