//! # Attendance Pallet
//!
//! A pallet with minimal functionality to help developers understand the essential components of
//! writing a FRAME pallet. It is typically used in beginner tutorials or in Substrate template
//! nodes as a starting point for creating a new pallet and **not meant to be used in production**.
//!
//! ## Overview
//!
//! This template pallet contains basic examples of:
//! - declaring a storage item that stores a single `u32` value
//! - declaring and using events
//! - declaring and using errors
//! - a dispatchable function that allows a user to set a new value to storage and emits an event
//!   upon success
//! - another dispatchable function that causes a custom error to be thrown
//!
//! Each pallet section is annotated with an attribute using the `#[pallet::...]` procedural macro.
//! This macro generates the necessary code for a pallet to be aggregated into a FRAME runtime.
//!
//! Learn more about FRAME macros [here](https://docs.substrate.io/reference/frame-macros/).
//!
//! ### Pallet Sections
//!
//! The pallet sections in this template are:
//!
//! - A **configuration trait** that defines the types and parameters which the pallet depends on
//!   (denoted by the `#[pallet::config]` attribute). See: [`Config`].
//! - A **means to store pallet-specific data** (denoted by the `#[pallet::storage]` attribute).
//!   See: [`storage_types`].
//! - A **declaration of the events** this pallet emits (denoted by the `#[pallet::event]`
//!   attribute). See: [`Event`].
//! - A **declaration of the errors** that this pallet can throw (denoted by the `#[pallet::error]`
//!   attribute). See: [`Error`].
//! - A **set of dispatchable functions** that define the pallet's functionality (denoted by the
//!   `#[pallet::call]` attribute). See: [`dispatchables`].
//!
//! Run `cargo doc --package pallet-template --open` to view this pallet's documentation.

// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
    // Import various useful types required by all FRAME pallets.
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::{ensure_signed, pallet_prelude::*};
    use sp_core::crypto::{Pair, Public, Signature};
    use sp_core::Hasher;
    use sp_runtime::app_crypto::ByteArray;

    pub trait Mintable<T> {
        fn mint(account: &T);
    }

    // The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
    // (`Call`s) in this pallet.
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    type Challenge<T> = BoundedVec<u8, <T as pallet::Config>::MaxGeohashLength>;
    type RawPublicKey = BoundedVec<u8, ConstU32<32>>;
    type RawSignature = BoundedVec<u8, ConstU32<64>>;
    type RawVerifyingKey = BoundedVec<u8, ConstU32<64>>;
    type RawProof = BoundedVec<u8, ConstU32<64>>;

    /// The pallet's configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: WeightInfo;
        /// Payload hasher
        type PayloadHasher: Hasher<Out = Self::Hash>;
        /// Public Key of Oracle
        type PublicKeyOfOracle: Public;
        /// Signature
        type Signature: Signature;
        /// Verification
        type Verify: Pair<Public = Self::PublicKeyOfOracle, Signature = Self::Signature>;
        /// Mint to Account
        type Mint: Mintable<Self::AccountId>;
        /// Maximum length allowed for geohash
        type MaxGeohashLength: Get<u32>;
    }

    #[pallet::storage]
    pub type Challenges<T: Config> = StorageMap<_, Blake2_128Concat, Challenge<T>, bool>;

    #[pallet::storage]
    pub type Oracle<T: Config> = StorageValue<_, RawPublicKey>;

    #[pallet::storage]
    pub type Submissions<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, Challenge<T>, Blake2_128Concat, T::AccountId, bool>;

    #[pallet::storage]
    pub type ProofVerifyingKey<T: Config> = StorageValue<_, RawVerifyingKey>;

    /// Events that functions in this pallet can emit.
    ///
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ChallengeCreated {
            who: T::AccountId,
            challenge: Challenge<T>,
        },
        SubmissionAccepted {
            who: T::AccountId,
            challenge: Challenge<T>,
            signature: RawSignature,
        },
    }

    /// Errors that can be returned by this pallet.
    ///
    #[pallet::error]
    pub enum Error<T> {
        InvalidGeohash,
        InvalidPublicKey,
        InvalidSignature,
        AlreadySubmitted,
        InvalidProof,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn create_challenge(origin: OriginFor<T>, challenge: Challenge<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Create a challenge
            ensure!(Self::valid_geohash(&challenge), Error::<T>::InvalidGeohash);
            ensure!(
                Challenges::<T>::contains_key(&challenge) == false,
                Error::<T>::InvalidGeohash
            );
            // Store the validated geohash
            Challenges::<T>::insert(challenge.clone(), true);

            Self::deposit_event(Event::ChallengeCreated { who, challenge });
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn submission_with_signature(
            origin: OriginFor<T>,
            challenge: Challenge<T>,
            location: Challenge<T>,
            signature: RawSignature,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                !Submissions::<T>::contains_key(&challenge, &who),
                Error::<T>::AlreadySubmitted
            );
            ensure!(
                Self::geohash_in_geohash(&location, &challenge),
                Error::<T>::InvalidGeohash
            );

            let message = T::PayloadHasher::hash(&location);
            let public_key = Oracle::<T>::get().expect("oracle key");
            let public_key = T::PublicKeyOfOracle::from_slice(&public_key)
                .map_err(|_| Error::<T>::InvalidPublicKey)?;

            ensure!(
                T::Verify::verify(
                    &T::Signature::from_slice(&signature)
                        .map_err(|_| Error::<T>::InvalidSignature)?,
                    message,
                    &public_key
                ),
                Error::<T>::InvalidSignature
            );

            T::Mint::mint(&who);
            Submissions::<T>::insert(challenge.clone(), who.clone(), true);

            Self::deposit_event(Event::SubmissionAccepted {
                who,
                challenge,
                signature,
            });

            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        pub fn set_oracle_public_key(
            origin: OriginFor<T>,
            public_key: RawPublicKey,
        ) -> DispatchResult {
            ensure_root(origin)?;
            Oracle::<T>::put(public_key);
            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(0)]
        pub fn submission_with_proof(
            origin: OriginFor<T>,
            challenge: Challenge<T>,
            proof: RawProof,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                Self::verify_zkp(&proof, &challenge),
                Error::<T>::InvalidProof
            );
            T::Mint::mint(&who);
            Submissions::<T>::insert(challenge.clone(), who.clone(), true);

            Ok(())
        }
    }

    use ark_bn254::Bn254;
    use ark_bn254::Fr;
    use ark_groth16::Groth16;
    use ark_groth16::{Proof, VerifyingKey};
    use ark_serialize::CanonicalDeserialize;
    use ark_snark::SNARK;

    impl<T: Config> Pallet<T> {
        pub fn valid_geohash(geohash: &Challenge<T>) -> bool {
            geohash
                .iter()
                .all(|c| "0123456789bcdefghjkmnpqrstuvwxyz".contains(*c as char))
        }

        fn geohash_in_geohash(geohash: &Challenge<T>, challenge: &Challenge<T>) -> bool {
            geohash.starts_with(challenge)
        }

        fn verify_zkp(proof: &RawProof, challenge: &Challenge<T>) -> bool {
            let proof = Proof::<Bn254>::deserialize_uncompressed(proof.as_slice()).expect("proof");
            let verifying_key_bytes = ProofVerifyingKey::<T>::get().expect("verifying key");

            let verifying_key =
                VerifyingKey::deserialize_uncompressed(verifying_key_bytes.as_slice())
                    .expect("verifying key");

            let public_input: sp_runtime::Vec<Fr> =
                challenge.iter().map(|c| (*c as u64).into()).collect();

            Groth16::<Bn254>::verify(&verifying_key, &public_input, &proof).expect("verified")
        }
    }
}
