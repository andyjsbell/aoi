use ed25519_dalek::{Signer, SigningKey};
use oracle::{Hash, Key};
use rand::rngs::OsRng;

pub struct Ed25519;

impl oracle::Signer for Ed25519 {
    type Signature = Vec<u8>;

    fn sign(message: Hash, key: Key) -> Result<Self::Signature, String> {
        let signing_key = SigningKey::from_bytes(key.as_bytes());

        let signature = signing_key
            .try_sign(message.as_bytes())
            .map_err(|e| e.to_string())?;

        Ok(signature.to_vec())
    }

    fn generate_key() -> (Key, Key) {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        (
            Key::new(signing_key.to_bytes()),
            Key::new(signing_key.verifying_key().to_bytes()),
        )
    }
}
