use core::hash::Hasher as StdHasher;
use core::marker::PhantomData;

use crate::{self as pallet_attendance, Mintable};
use codec::Encode;
use frame_support::{derive_impl, parameter_types};
use sp_core::crypto::Dummy;
use sp_core::{Hasher, H256};
use sp_runtime::BuildStorage;

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        AttendanceModule: pallet_attendance,
    }
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;
}
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref MINTS: Mutex<HashMap<Vec<u8>, bool>> = Mutex::new(HashMap::new());
}
pub struct MockMinter<T>(PhantomData<T>);

impl<T> Mintable<T> for MockMinter<T>
where
    T: Encode,
{
    fn mint(account: &T) {
        let mut mints = MINTS.lock().unwrap();
        mints.insert(account.encode(), true);
    }
}
parameter_types! {
    pub const MaxGeohashLength: u32 = 12;
}

#[derive(Default)]
pub struct StdDummyHasher;
impl StdHasher for StdDummyHasher {
    fn finish(&self) -> u64 {
        0
    }

    fn write(&mut self, _bytes: &[u8]) {}
}

pub struct MockHasher;
impl Hasher for MockHasher {
    type Out = H256;
    type StdHasher = StdDummyHasher;
    const LENGTH: usize = 0;
    fn hash(_x: &[u8]) -> Self::Out {
        H256::default()
    }
}

impl pallet_attendance::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type MaxGeohashLength = MaxGeohashLength;
    type Mint = MockMinter<Self::AccountId>;
    type PublicKeyOfOracle = Dummy;
    type PayloadHasher = MockHasher;
    type Signature = Dummy;
    type Verify = Dummy;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into()
}
