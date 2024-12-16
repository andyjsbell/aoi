use ark_bn254::Bn254;
use ark_bn254::Fr;
use ark_ff::PrimeField;
use ark_groth16::Groth16;
use ark_r1cs_std::{fields::fp::FpVar, prelude::*};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_snark::SNARK;
use rand::CryptoRng;
use rand::RngCore;

#[derive(Clone, Default)]
pub struct CompareCircuit<F: PrimeField> {
    pub shorter: Option<Vec<F>>,
    pub larger: Option<Vec<F>>,
}

impl CompareCircuit<Fr> {
    pub fn new(shorter: Vec<Fr>, larger: Vec<Fr>) -> Self {
        Self {
            shorter: Some(shorter),
            larger: Some(larger),
        }
    }

    pub fn new_from_str<'a>(shorter: &'a str, larger: &'a str) -> Self {
        Self {
            shorter: Some(PrimeString::<Fr>::from(shorter).into()),
            larger: Some(PrimeString::<Fr>::from(larger).into()),
        }
    }
}

pub fn setup_groth16<R: RngCore + CryptoRng>(
    rng: &mut R, circuit: CompareCircuit<Fr>
) -> Result<
    (
        ark_groth16::ProvingKey<Bn254>,
        ark_groth16::VerifyingKey<Bn254>,
    ),
    SynthesisError,
> {
    Groth16::<Bn254>::circuit_specific_setup(circuit, rng)
}

pub fn create_proof<R: RngCore + CryptoRng>(
    pk: &ark_groth16::ProvingKey<Bn254>,
    circuit: CompareCircuit<Fr>,
    rng: &mut R,
) -> Result<ark_groth16::Proof<Bn254>, SynthesisError> {
    Groth16::<Bn254>::prove(pk, circuit, rng)
}

pub fn verify_proof<'a>(
    vk: &ark_groth16::VerifyingKey<Bn254>,
    public_inputs: &'a str,
    proof: &ark_groth16::Proof<Bn254>,
) -> Result<bool, SynthesisError> {
    let public_inputs = &Vec::<Fr>::from(PrimeString::<Fr>::from(public_inputs));
    Groth16::<Bn254>::verify(vk, public_inputs, proof)
}

impl<F: PrimeField> ConstraintSynthesizer<F> for CompareCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let shorter = self.shorter.ok_or(SynthesisError::AssignmentMissing)?;
        let larger = self.larger.ok_or(SynthesisError::AssignmentMissing)?;

        if shorter.is_empty() || larger.is_empty() || shorter.len() > larger.len() {
            return Err(SynthesisError::Unsatisfiable);
        }

        // Public
        let shorter_vars = shorter
            .iter()
            .map(|&val| FpVar::new_input(cs.clone(), || Ok(val)))
            .collect::<Result<Vec<_>, _>>()?;

        // Witness
        let larger_vars = larger
            .iter()
            .take(shorter.len())
            .map(|&val| FpVar::new_witness(cs.clone(), || Ok(val)))
            .collect::<Result<Vec<_>, _>>()?;

        for (shorter_var, larger_var) in shorter_vars.iter().zip(larger_vars.iter()) {
            larger_var.enforce_equal(shorter_var)?;
        }
        Ok(())
    }
}

// Generate a vector of prime field values for a string
#[derive(Clone)]
struct PrimeString<F: PrimeField>(Vec<F>);
impl<'a, F: PrimeField> From<&'a str> for PrimeString<F> {
    fn from(value: &'a str) -> Self {
        Self(
            value
                .as_bytes()
                .iter()
                .map(|c| (*c as u64).into())
                .collect(),
        )
    }
}

impl<F: PrimeField> From<PrimeString<F>> for Vec<F> {
    fn from(value: PrimeString<F>) -> Self {
        value.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use rand::thread_rng;

    use super::*;

    fn prove_verify_starts_with<'a>(small: &'a str, large: &'a str) {
        let circuit = CompareCircuit::new_from_str(small, large);
        let rng = &mut thread_rng();

        let (pk, vk) = setup_groth16(rng, circuit.clone()).expect("setup failed");
        let proof = create_proof(&pk, circuit, rng).expect("proof not generated");
        let verified = verify_proof(&vk, small, &proof).expect("verification failed");

        assert!(verified, "this can't be verified");
    }

    #[test]
    fn test_new_from_str() {
        let circuit = CompareCircuit::new_from_str("abc", "def");
        assert_eq!(
            circuit.shorter,
            Some(vec![Fr::from(97), Fr::from(98), Fr::from(99)])
        );
        assert_eq!(
            circuit.larger,
            Some(vec![Fr::from(100), Fr::from(101), Fr::from(102)])
        );
    }

    #[test]
    fn test_empty_shorter() {
        let result = std::panic::catch_unwind(|| {
            prove_verify_starts_with("", "bcdef");
        });
        assert!(
            result.is_err(),
            "Expected panic, but the code did not panic"
        );
    }

    #[test]
    fn test_empty_larger() {
        let result = std::panic::catch_unwind(|| {
            prove_verify_starts_with("bcd", "");
        });
        assert!(
            result.is_err(),
            "Expected panic, but the code did not panic"
        );
    }

    #[test]
    fn test_equal_strings() {
        prove_verify_starts_with("abc", "abcdef");
    }

    #[test]
    fn test_shorter_longer_than_larger() {
        let result = std::panic::catch_unwind(|| {
            prove_verify_starts_with("abcdef", "abc");
        });
        assert!(
            result.is_err(),
            "Expected panic, but the code did not panic"
        );
    }
}
