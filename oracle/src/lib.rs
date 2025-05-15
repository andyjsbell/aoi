use serde::ser::Serialize;

/// A 32-byte cryptographic key used for operations like signing.
///
/// This represents either a public or private key depending on context.
/// Internally stored as a fixed-size byte array.
#[derive(Clone, Copy)]
pub struct Key([u8; 32]);

/// A 32-byte cryptographic hash value.
///
/// Represents the output of a hash function applied to arbitrary data.
/// Used for operations like signature generation.
#[derive(Clone, Copy)]
pub struct Hash([u8; 32]);

impl Key {
    /// Creates a new Key from a 32-byte array.
    ///
    /// # Arguments
    /// * `bytes` - A 32-byte array containing the key data
    ///
    /// # Returns
    /// A new Key instance containing the provided bytes
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Returns a reference to the underlying byte array.
    ///
    /// # Returns
    /// A reference to the 32-byte array storing the key
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl Hash {
    /// Creates a new Hash from a 32-byte array.
    ///
    /// # Arguments
    /// * `bytes` - A 32-byte array containing the hash data
    ///
    /// # Returns
    /// A new Hash instance containing the provided bytes
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Returns a reference to the underlying byte array.
    ///
    /// # Returns
    /// A reference to the 32-byte array storing the hash
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// Errors that can occur during location operations.
///
/// This enum represents the various ways that acquiring or
/// formatting location data can fail.
#[derive(Error, Debug)]
pub enum LocationError {
    /// Failed to obtain the current geographical location.
    ///
    /// This typically occurs when network requests to location services fail
    /// or when location data is unavailable.
    #[error("failed to locate")]
    Location,
    
    /// Failed to generate the location output in the required format.
    ///
    /// This happens when the raw location data cannot be converted to
    /// the required output format (e.g., geohash).
    ///
    /// # Fields
    /// * String - A description of what went wrong during formatting
    #[error("failed to generate output: {0}")]
    Output(String),
}

/// Trait for obtaining geographical location data.
///
/// Implementors of this trait can provide location data from various sources
/// (GPS, IP geolocation, etc.) and in different formats.
#[async_trait::async_trait]
pub trait Location {
    /// The type representing location data.
    ///
    /// Must be able to be represented as a byte slice for hashing and signing.
    type Output: AsRef<[u8]>;
    
    /// Asynchronously obtains the current geographical location.
    ///
    /// # Arguments
    /// * `accuracy` - The desired accuracy level for the location data.
    ///   Higher values typically mean more precise location data.
    ///   For geohash implementations, this often represents the length of the hash.
    ///
    /// # Returns
    /// * `Result<Self::Output, LocationError>` - The location data if successful,
    ///   or an error if obtaining the location failed.
    async fn current_location(accuracy: u8) -> Result<Self::Output, LocationError>;
}

/// Trait for cryptographic hashing functionality.
///
/// Implementors of this trait provide methods to hash arbitrary data
/// into fixed-size Hash objects.
pub trait Hasher {
    /// Computes a cryptographic hash of the provided message.
    ///
    /// # Arguments
    /// * `message` - The data to hash, which can be any type that can be
    ///   represented as a byte slice.
    ///
    /// # Returns
    /// A 32-byte Hash of the input data.
    fn hash<T>(message: T) -> Hash
    where
        T: AsRef<[u8]>;
}

use thiserror::Error;

/// Errors that can occur during cryptographic signing operations.
///
/// This enum represents the various ways that creating signatures
/// can fail.
#[derive(Error, Debug)]
pub enum SignerError {
    /// Failed to create a signature.
    ///
    /// This typically occurs when the signing operation fails due to
    /// invalid keys or internal cryptographic errors.
    ///
    /// # Fields
    /// * String - A description of what went wrong during signing
    #[error("signature failed: {0}")]
    SignatureFailed(String),
}

/// Trait for cryptographic signing operations.
///
/// Implementors of this trait provide methods to digitally sign data
/// and generate cryptographic key pairs.
pub trait Signer {
    /// The type representing a cryptographic signature.
    ///
    /// Must be serializable for storage or transmission.
    type Signature: Serialize;
    
    /// Signs a message hash using the provided key.
    ///
    /// # Arguments
    /// * `message` - The hash of the message to sign
    /// * `key` - The private key to use for signing
    ///
    /// # Returns
    /// * `Result<Self::Signature, SignerError>` - The signature if successful,
    ///   or an error if signing failed.
    fn sign(message: Hash, key: Key) -> Result<Self::Signature, SignerError>;
    
    /// Generates a new cryptographic key pair.
    ///
    /// # Returns
    /// A tuple containing (private_key, public_key)
    fn generate_key() -> (Key, Key);
}

/// Helper function to obtain location data using the specified Location implementation.
///
/// This is a convenience wrapper around the Location trait's current_location method.
///
/// # Type Parameters
/// * `L` - A type that implements the Location trait
///
/// # Arguments
/// * `accuracy` - The desired accuracy level for the location data
///
/// # Returns
/// * `Result<L::Output, LocationError>` - The location data if successful,
///   or an error if obtaining the location failed
pub async fn location<L>(accuracy: u8) -> Result<L::Output, LocationError>
where
    L: Location,
{
    L::current_location(accuracy).await
}

/// Signs location data using specified cryptographic components.
///
/// This function composes the hashing and signing operations:
/// 1. Converts the location data to bytes
/// 2. Hashes the bytes using the specified Hasher
/// 3. Signs the hash using the specified Signer and key
///
/// # Type Parameters
/// * `L` - A type that implements the Location trait
/// * `S` - A type that implements the Signer trait
/// * `H` - A type that implements the Hasher trait
///
/// # Arguments
/// * `key` - The private key to use for signing
/// * `location` - The location data to sign
///
/// # Returns
/// * `Result<S::Signature, SignerError>` - The signature if successful,
///   or an error if signing failed
pub async fn sign_location<L, S, H>(
    key: Key,
    location: L::Output,
) -> Result<S::Signature, SignerError>
where
    L: Location,
    S: Signer,
    H: Hasher,
{
    S::sign(H::hash(location.as_ref()), key)
}
