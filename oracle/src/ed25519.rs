//! Ed25519 digital signature implementation.
//!
//! This module provides an implementation of the `Signer` trait
//! using the Ed25519 elliptic curve digital signature algorithm.

use ed25519_dalek::{Signer, SigningKey};
use oracle::{Hash, Key, SignerError};
use rand::rngs::OsRng;

/// Implementation of the `Signer` trait using the Ed25519 signature algorithm.
///
/// Ed25519 is a state-of-the-art elliptic curve signature scheme that provides
/// strong security and performance characteristics. It's widely used in
/// cryptographic applications requiring digital signatures.
pub struct Ed25519;

impl oracle::Signer for Ed25519 {
    /// The type of signature produced by this implementation.
    ///
    /// Ed25519 signatures are binary data represented as a byte vector.
    type Signature = Vec<u8>;

    /// Signs a message hash using an Ed25519 private key.
    ///
    /// # Arguments
    ///
    /// * `message` - The hash of the message to sign
    /// * `key` - The private key to use for signing
    ///
    /// # Returns
    ///
    /// * `Result<Vec<u8>, SignerError>` - The Ed25519 signature as a byte vector if successful,
    ///   or a SignerError if signing failed.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The provided key is invalid for Ed25519 signing
    /// - The internal signing operation fails
    fn sign(message: Hash, key: Key) -> Result<Self::Signature, SignerError> {
        let signing_key = SigningKey::from_bytes(key.as_bytes());

        let signature = signing_key
            .try_sign(message.as_bytes())
            .map_err(|e| SignerError::SignatureFailed(e.to_string()))?;

        Ok(signature.to_vec())
    }

    /// Generates a new Ed25519 key pair for signing and verification.
    ///
    /// This function generates a cryptographically secure random Ed25519 key pair
    /// using the operating system's random number generator.
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - The private key (signing key)
    /// - The public key (verification key)
    ///
    /// # Examples
    ///
    /// ```
    /// let (private_key, public_key) = Ed25519::generate_key();
    /// // Use private_key for signing
    /// // Share public_key for verification
    /// ```
    fn generate_key() -> (Key, Key) {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        (
            Key::new(signing_key.to_bytes()),
            Key::new(signing_key.verifying_key().to_bytes()),
        )
    }
}
