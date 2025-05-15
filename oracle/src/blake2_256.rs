use sp_io::hashing::blake2_256;
pub struct Blake2_256;
use oracle::{Hash, Hasher};

impl Hasher for Blake2_256 {
    fn hash<T>(message: T) -> Hash
    where
        T: AsRef<[u8]>,
    {
        Hash::new(blake2_256(message.as_ref()))
    }
}
