//! Shared types used across the ZK verifier contract.

use soroban_sdk::{contracterror, contracttype, Bytes};

/// Raw proof bytes submitted by the caller.
#[contracttype]
#[derive(Clone, Debug)]
pub struct ProofBytes {
    /// Serialised proof data (curve-point encoding).
    pub data: Bytes,
}

/// Public inputs that accompany a zero-knowledge proof.
#[contracttype]
#[derive(Clone, Debug)]
pub struct PublicInputs {
    /// ABI-encoded public input scalars.
    pub data: Bytes,
}

/// A verification key used to validate a specific circuit.
#[contracttype]
#[derive(Clone, Debug)]
pub struct VerificationKey {
    /// Serialised verification key (scheme-specific encoding).
    pub data: Bytes,
    /// Human-readable circuit identifier.
    pub circuit_id: Bytes,
}

/// Errors returned by verifier entry points.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum VerifierError {
    /// The supplied proof bytes are malformed or cannot be deserialised.
    InvalidProof = 1,
    /// The proof verification check failed (pairing or polynomial identity).
    VerificationFailed = 2,
    /// The supplied public inputs are malformed.
    InvalidPublicInputs = 3,
    /// The requested verification key is not registered.
    UnknownVerificationKey = 4,
    /// The caller is not authorised to perform this action.
    Unauthorized = 5,
    /// The nullifier has already been spent.
    NullifierSpent = 6,
    /// Feature not yet implemented.
    NotImplemented = 99,
}
