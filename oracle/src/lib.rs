use serde::ser::Serialize;

#[derive(Clone, Copy)]
pub struct Key([u8; 32]);

#[derive(Clone, Copy)]
pub struct Hash([u8; 32]);

impl Key {
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl Hash {
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

#[async_trait::async_trait]
pub trait Location {
    type Output: AsRef<[u8]>;
    async fn current_location(accuracy: u8) -> Result<Self::Output, String>;
}

pub trait Hasher {
    fn hash<T>(message: T) -> Result<Hash, String>
    where
        T: AsRef<[u8]>;
}

pub trait Signer {
    type Signature: Serialize;
    fn sign(message: Hash, key: Key) -> Result<Self::Signature, String>;
    fn generate_key() -> (Key, Key);
}

pub async fn sign_location<L, S, H>(key: Key, accuracy: u8) -> Result<(L::Output, S::Signature), String>
where
    L: Location,
    S: Signer,
    H: Hasher,
{
    let location = L::current_location(accuracy).await?;
    let signature = S::sign(H::hash(location.as_ref())?, key)?;
    Ok((location, signature))
}
