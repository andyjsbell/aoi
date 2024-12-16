mod tests {
    use crate::{mock::*, Challenges, Error};
    use frame_support::{assert_noop, assert_ok};
    use sp_core::{crypto::Dummy, Pair};
    use sp_runtime::BoundedVec;

    const ALICE: u64 = 1;

    #[derive(Clone)]
    struct Geohash(&'static str);
    impl From<Geohash> for BoundedVec<u8, <Test as crate::Config>::MaxGeohashLength> {
        fn from(value: Geohash) -> Self {
            BoundedVec::try_from(value.0.as_bytes().to_vec())
                .expect("Failed to convert geohash string to bounded vector")
        }
    }
    #[test]
    fn test_valid_geohash() {
        // Test valid geohashes
        let valid_geohashes = ["bcd", "ezs42", "u4pruydqqvj"];
        for geohash in valid_geohashes {
            assert!(
                AttendanceModule::valid_geohash(&Geohash(geohash).into()),
                "Geohash '{}' should be valid",
                geohash
            );
        }

        // Test invalid geohashes
        let invalid_geohashes = ["abc", "!@#", "ABC"];
        for geohash in invalid_geohashes {
            assert!(
                !AttendanceModule::valid_geohash(&Geohash(geohash).into()),
                "Geohash '{}' should be invalid",
                geohash
            );
        }
    }

    #[test]
    fn create_challenge() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);
            let geohash = BoundedVec::<u8, MaxGeohashLength>::from(Geohash("bcd"));
            assert_ok!(AttendanceModule::create_challenge(
                RuntimeOrigin::signed(ALICE),
                geohash.clone()
            ));
            assert!(Challenges::<Test>::contains_key(geohash.clone()));
            assert_noop!(
                AttendanceModule::create_challenge(RuntimeOrigin::signed(ALICE), geohash),
                Error::<Test>::InvalidGeohash
            );
        });
    }

    #[test]
    fn submit_valid_geohash_for_challenge() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            assert_ok!(AttendanceModule::set_oracle_public_key(
                RuntimeOrigin::root(),
                Dummy::default().to_raw_vec().try_into().expect("")
            ));

            assert_ok!(AttendanceModule::create_challenge(
                RuntimeOrigin::signed(ALICE),
                Geohash("bcd").into()
            ));

            let signature = Dummy::default();

            assert_ok!(AttendanceModule::submission_with_signature(
                RuntimeOrigin::signed(ALICE),
                Geohash("bcd").into(),
                Geohash("bcdefg").into(),
                signature
                    .to_raw_vec()
                    .try_into()
                    .expect("signature to vector"),
            ));
        });
    }

    #[test]
    fn submit_proof_for_challenge() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            assert_ok!(AttendanceModule::create_challenge(
                RuntimeOrigin::signed(ALICE),
                Geohash("bcd").into()
            ));

            
            let signature = Dummy::default();

            assert_ok!(AttendanceModule::submission_with_signature(
                RuntimeOrigin::signed(ALICE),
                Geohash("bcd").into(),
                Geohash("bcdefg").into(),
                signature
                    .to_raw_vec()
                    .try_into()
                    .expect("signature to vector"),
            ));
        });
    }
    #[test]
    fn set_oracle_public_key() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);
            assert_ok!(AttendanceModule::set_oracle_public_key(
                RuntimeOrigin::root(),
                Dummy::default().to_raw_vec().try_into().expect("")
            ));
        });
    }
}
