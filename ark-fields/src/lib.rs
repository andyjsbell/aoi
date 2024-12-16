use ark_ff::{Fp256, MontBackend, UniformRand};
use ark_test_curves::{bls12_381::{Fr, G1Projective}, PrimeField, PrimeGroup};
use std::{hash::Hash, ops::Mul};
use ark_test_curves::CurveGroup;
use sha2::{Sha256, Digest};
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
use ark_test_curves::bls12_381::FrConfig;
use ark_test_curves::bls12_381::g1::Config;
use ark_test_curves::short_weierstrass::Projective;

struct Keypair {
    private_key: Fp256<MontBackend<FrConfig, 4>>,
    public_key: Projective<Config>,
}

struct Signature {
    R: Projective<Config>,
    z: Fp256<MontBackend<FrConfig, 4>>,
}

pub fn generate_key_pair() -> Keypair {
    let generator = G1Projective::generator();
    let mut rng = rand::thread_rng();
    let private_key = Fr::rand(&mut rng);
    let public_key = generator * private_key;
    Keypair {
        private_key,
        public_key,
    }
}

pub fn hash(message: Vec<u8>, public_key: Projective<Config>, R: Projective<Config>) -> Vec<u8> {
    // c = Hash(Public key + message + R)
    let mut hasher = Sha256::new();
    let mut public_key_bytes = Vec::new();
    public_key.serialize_uncompressed(&mut public_key_bytes).unwrap();
    hasher.update(public_key_bytes);
    hasher.update(message);
    let mut R_bytes = Vec::new();
    R.serialize_uncompressed(&mut R_bytes).unwrap();
    hasher.update(R_bytes);

    hasher.finalize().to_vec()
}

pub fn sign(keypair: &Keypair, message: Vec<u8>) -> Signature {
    let generator = G1Projective::generator();
    let mut rng = rand::thread_rng();
    let r = Fr::rand(&mut rng);
    let R = generator * r;
    let c = hash(message, keypair.public_key, R);
    let c = Fr::from_be_bytes_mod_order(c.as_slice());
    // z = (r + c * private_key) % curve_order
    let z = (r + c * keypair.private_key);

    Signature {
        R, z
    }
}

pub fn verify(public_key: Projective<Config>, signature: &Signature, message: Vec<u8>) -> bool {
    let generator = G1Projective::generator();
    let c = hash(message, public_key, signature.R);
    let c = Fr::from_be_bytes_mod_order(c.as_slice());
    signature.R + (public_key * c) == generator * signature.z
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn g1_works() {
        let message = "hello".as_bytes();
        let keypair = generate_key_pair();
        let signature = sign(&keypair, message.into());

        assert!(verify(keypair.public_key, &signature, message.into()));
    }
}
