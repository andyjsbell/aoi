use std::env;
use thiserror::Error;

/// Errors that can occur during environment and key-related operations.
///
/// This enum covers errors related to reading environment variables,
/// parsing hex strings, and other key management operations.
#[derive(Error, Debug, PartialEq)]
pub enum EnvError {
    /// The provided hex string has an invalid length.
    ///
    /// This error occurs when the hex string length doesn't match
    /// the expected length for the target array.
    #[error("invalid hex string length")]
    InvalidHexLength,

    /// The required environment variable was not found.
    ///
    /// This error occurs when attempting to read an environment
    /// variable that doesn't exist or isn't accessible.
    #[error("environment variable not found")]
    VarNotFound,

    /// Failed to parse the provided hex string.
    ///
    /// This error occurs when the string contains invalid characters
    /// or doesn't follow the hex format requirements.
    ///
    /// # Fields
    /// * String - A description of what went wrong during parsing
    #[error("failed to parse hex string: {0}")]
    HexParseError(String),
}

/// Environment variable name used to store the oracle's private key.
///
/// When set, this environment variable should contain a hex-encoded
/// 32-byte private key used for signing operations.
const ENV_ORACLE_KEY: &str = "ORACLE_KEY";

/// Converts a hexadecimal string to a fixed-size byte array.
///
/// This function handles both raw hex strings and those with a "0x" prefix.
/// It ensures the string length matches the expected output array size
/// before attempting conversion.
///
/// # Type Parameters
/// * `N` - The size of the output array in bytes
///
/// # Arguments
/// * `hex_string` - The hexadecimal string to convert (with or without "0x" prefix)
///
/// # Returns
/// * `Result<[u8; N], EnvError>` - The byte array if successful, or an error if:
///   - The string length doesn't match the expected array size (2 hex chars per byte)
///   - The string contains invalid hexadecimal characters
///
/// # Examples
/// ```
/// let bytes = try_hex_to_array::<2>("0102".to_string()).unwrap();
/// assert_eq!(bytes, [1, 2]);
///
/// let bytes = try_hex_to_array::<2>("0x0102".to_string()).unwrap();
/// assert_eq!(bytes, [1, 2]);
/// ```
pub(crate) fn try_hex_to_array<const N: usize>(hex_string: String) -> Result<[u8; N], EnvError> {
    let hex_string = hex_string.strip_prefix("0x").unwrap_or(&hex_string);
    if hex_string.len() != N * 2 {
        return Err(EnvError::InvalidHexLength);
    }
    let mut out = [0u8; N];
    hex::decode_to_slice(hex_string, &mut out)
        .map_err(|e| EnvError::HexParseError(e.to_string()))?;
    Ok(out)
}

/// Converts a byte array or slice to a hexadecimal string.
///
/// This function takes any type that can be treated as a byte slice
/// and returns its hexadecimal string representation.
///
/// # Type Parameters
/// * `T` - Any type that can be referenced as a byte slice
///
/// # Arguments
/// * `array` - The bytes to convert to a hex string
///
/// # Returns
/// A hexadecimal string representing the input bytes
///
/// # Examples
/// ```
/// let hex = array_to_hex([0x01, 0x02, 0x03]);
/// assert_eq!(hex, "010203");
/// ```
pub(crate) fn array_to_hex<T: AsRef<[u8]>>(array: T) -> String {
    hex::encode(array)
}

/// Attempts to read a 32-byte key from the environment variable.
///
/// This function reads the ORACLE_KEY environment variable, expecting
/// it to contain a valid hexadecimal string representing a 32-byte key.
///
/// # Returns
/// * `Result<[u8; 32], EnvError>` - The 32-byte key if successful, or an error if:
///   - The environment variable is not set or not accessible
///   - The variable's value is not a valid hexadecimal string
///   - The hex string doesn't decode to exactly 32 bytes
///
/// # Examples
/// ```
/// // Assuming ORACLE_KEY environment variable is set to a valid hex string
/// let key = try_key_from_environment().unwrap();
/// // Use the key for cryptographic operations
/// ```
pub(crate) fn try_key_from_environment() -> Result<[u8; 32], EnvError> {
    try_hex_to_array(env::var(ENV_ORACLE_KEY).map_err(|_| EnvError::VarNotFound)?)
}

#[test]
fn test_try_key_from_environment() {
    let secret_key = format!("{}2a", "0".repeat(62));
    env::set_var(ENV_ORACLE_KEY, secret_key);
    let mut array = [0; 32];
    array[31] = 0x2a;
    assert_eq!(try_key_from_environment(), Ok(array));
    env::remove_var(ENV_ORACLE_KEY);
}

#[test]
fn test_try_hex_to_array() {
    // Valid hex string
    let result = try_hex_to_array::<2>("0102".to_string());
    assert_eq!(result, Ok([1, 2]));

    // Valid with 0x prefix
    let result = try_hex_to_array::<2>("0x0102".to_string());
    assert_eq!(result, Ok([1, 2]));

    // Invalid length
    let result = try_hex_to_array::<2>("010".to_string());
    assert_eq!(result, Err(EnvError::InvalidHexLength));

    // Invalid hex characters
    let result = try_hex_to_array::<2>("01zz".to_string());
    assert_eq!(
        result,
        Err(EnvError::HexParseError(
            "Invalid character 'z' at position 2".to_string()
        ))
    );
}
