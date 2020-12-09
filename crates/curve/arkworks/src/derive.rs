//! Impl ops for curves
#![macro_use]

/// Paste pairing test
#[macro_export]
macro_rules! paste_test {
    ($curve:ident, $fq:ident) => {
        #[test]
        fn test_wasm_pairing() {
            let mut rng = test_rng();
            let a: G1Projective = rng.gen();
            let b: G2Projective = rng.gen();
            let s: Fr = rng.gen();

            let mut sa = a;
            sa.mul_assign(s);
            let mut sb = b;
            sb.mul_assign(s);

            let ans1 = <$curve as PairingEngine>::pairing(sa, b);
            let ans2 = <$curve as PairingEngine>::pairing(a, sb);
            let ans3 = <$curve as PairingEngine>::pairing(a, b).pow(s.into_repr());

            assert_eq!(ans1, ans2);
            assert_eq!(ans2, ans3);

            assert_ne!(ans1, $fq::one());
            assert_ne!(ans2, $fq::one());
            assert_ne!(ans3, $fq::one());

            assert_eq!(ans1.pow(Fr::characteristic()), $fq::one());
            assert_eq!(ans2.pow(Fr::characteristic()), $fq::one());
            assert_eq!(ans3.pow(Fr::characteristic()), $fq::one());
        }
    };
}

mod bls12_377 {
    use crate::CurveBasicOperations;
    use ark_bls12_377::{Bls12_377, Fq12, Fr, G1Projective, G2Projective};
    use ark_ec::PairingEngine;
    use ark_ff::{
        fields::{Field, PrimeField},
        test_rng, One,
    };
    use ark_std::{ops::MulAssign, vec::Vec};
    use rand::Rng;
    use rustc_hex::FromHex;

    impl CurveBasicOperations for Bls12_377 {
        const SCALAR_FIELD: &'static str = "";
        const PRIME_FIELD: &'static str = "258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458177";
        const G1_LEN: usize = 97;
        const G2_LEN: usize = 193;
        const SCALAR_LEN: usize = 32;
    }

    paste_test!(Bls12_377, Fq12);
}

mod bls12_381 {
    use crate::CurveBasicOperations;
    use ark_bls12_381::{Bls12_381, Fq12, Fr, G1Projective, G2Projective};
    use ark_ec::PairingEngine;
    use ark_ff::{test_rng, Field, One, PrimeField};
    use ark_std::{ops::MulAssign, vec::Vec};
    use rand::Rng;
    use rustc_hex::FromHex;

    impl CurveBasicOperations for Bls12_381 {
        const SCALAR_FIELD: &'static str =
            "52435875175126190479447740508185965837690552500527637822603658699938581184513";
        const PRIME_FIELD: &'static str = "4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787";
        const G1_LEN: usize = 97;
        const G2_LEN: usize = 193;
        const SCALAR_LEN: usize = 32;
    }

    paste_test!(Bls12_381, Fq12);
}

mod bn254 {
    use crate::CurveBasicOperations;
    use ark_bn254::{Bn254, Fq12, Fr, G1Projective, G2Projective};
    use ark_ec::PairingEngine;
    use ark_ff::{test_rng, Field, One, PrimeField};
    use ark_std::{ops::MulAssign, vec::Vec};
    use rand::Rng;
    use rustc_hex::FromHex;

    impl CurveBasicOperations for Bn254 {
        const SCALAR_FIELD: &'static str =
            "21888242871839275222246405745257275088548364400416034343698204186575808495617";
        const PRIME_FIELD: &'static str =
            "21888242871839275222246405745257275088696311157297823662689037894645226208583";
        const G1_LEN: usize = 65;
        const G2_LEN: usize = 129;
        const SCALAR_LEN: usize = 32;
    }

    paste_test!(Bn254, Fq12);
}

mod bw6_761 {
    use crate::CurveBasicOperations;
    use ark_bw6_761::{Fq6, Fr, G1Projective, G2Projective, BW6_761};
    use ark_ec::PairingEngine;
    use ark_ff::{test_rng, Field, One, PrimeField};
    use ark_std::{ops::MulAssign, vec::Vec};
    use rand::Rng;
    use rustc_hex::FromHex;

    impl CurveBasicOperations for BW6_761 {
        const SCALAR_FIELD: &'static str = "";
        const PRIME_FIELD: &'static str = "";
        const G1_LEN: usize = 193;
        const G2_LEN: usize = 193;
        const SCALAR_LEN: usize = 48;
    }

    paste_test!(BW6_761, Fq6);
}

mod cp6_782 {
    use crate::CurveBasicOperations;
    use ark_cp6_782::{Fq6, Fr, G1Projective, G2Projective, CP6_782};
    use ark_ec::PairingEngine;
    use ark_ff::{test_rng, Field, One, PrimeField};
    use ark_std::{ops::MulAssign, vec::Vec};
    use rand::Rng;
    use rustc_hex::FromHex;

    impl CurveBasicOperations for CP6_782 {
        const SCALAR_FIELD: &'static str = "";
        const PRIME_FIELD: &'static str = "";
        const G1_LEN: usize = 209;
        const G2_LEN: usize = 625;
        const SCALAR_LEN: usize = 48;
    }

    paste_test!(CP6_782, Fq6);
}
