use std::env;

const ENV_ORACLE_KEY: &str = "ORACLE_KEY";

pub(crate) fn try_hex_to_array<const N: usize>(hex_string: String) -> Result<[u8; N], String> {
    let hex_string = hex_string.strip_prefix("0x").unwrap_or(&hex_string);
    assert!(hex_string.len() == N * 2, "invalid hex string length");
    let mut out = [0u8; N];
    hex::decode_to_slice(hex_string, &mut out).map_err(|e| e.to_string())?;
    Ok(out)
}

pub(crate) fn array_to_hex<T: AsRef<[u8]>>(array: T) -> String {
    hex::encode(array)
}

pub(crate) fn try_key_from_environment() -> Result<[u8; 32], String> {
    try_hex_to_array(env::var(ENV_ORACLE_KEY).map_err(|e| e.to_string())?)
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