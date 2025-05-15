//! Blake2-256 hash implementation.
//!
//! This module provides an implementation of the `Hasher` trait
//! using the Blake2-256 cryptographic hash function.

use sp_io::hashing::blake2_256;
use oracle::{Hash, Hasher};

/// Implementation of the `Hasher` trait using Blake2-256.
///
/// Blake2-256 is a cryptographic hash function that produces a 32-byte hash.
/// It's designed to be fast and secure, making it suitable for signing data.
pub struct Blake2_256;

impl Hasher for Blake2_256 {
    /// Computes a Blake2-256 hash of the provided message.
    ///
    /// # Arguments
    ///
    /// * `message` - The data to hash, which can be any type that can be
    ///   referenced as a byte slice.
    ///
    /// # Returns
    ///
    /// A 32-byte `Hash` containing the Blake2-256 hash of the input data.
    ///
    /// # Example
    ///
    /// ```
    /// let message = "Hello, world!";
    /// let hash = Blake2_256::hash(message);
    /// ```
    fn hash<T>(message: T) -> Hash
    where
        T: AsRef<[u8]>,
    {
        Hash::new(blake2_256(message.as_ref()))
    }
}
