//! Impl ops for curves

use rand::Rng;

use ark_ec::PairingEngine;
use ark_ff::{Field, One, PrimeField};
use ark_std::{ops::MulAssign, test_rng, vec::Vec};

use crate::CurveBasicOperations;

/// Paste pairing test
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
    use super::*;
    use ark_bls12_377::{Bls12_377, Fq12, Fr, G1Projective, G2Projective};

    impl CurveBasicOperations for Bls12_377 {
        const SCALAR_FIELD: &'static str =
            "8444461749428370424248824938781546531375899335154063827935233455917409239041";
        // const MODULUS: &'static str = "258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458177";
        const MODULUS: &'static [u8] = &[
            1, 0, 0, 0, 0, 192, 8, 133, 0, 0, 0, 48, 68, 93, 11, 23, 0, 72, 9, 186, 47, 98, 243,
            30, 143, 19, 245, 0, 243, 217, 34, 26, 59, 73, 161, 108, 192, 5, 59, 198, 234, 16, 197,
            23, 70, 58, 174, 1,
        ];
        const G1_LEN: usize = 97;
        const G2_LEN: usize = 193;
        const SCALAR_LEN: usize = 32;
        const CURVE_ID: u32 = 0x00;
    }

    paste_test!(Bls12_377, Fq12);
}

mod bls12_381 {
    use super::*;
    use ark_bls12_381::{Bls12_381, Fq12, Fr, G1Projective, G2Projective};

    impl CurveBasicOperations for Bls12_381 {
        const SCALAR_FIELD: &'static str =
            "52435875175126190479447740508185965837690552500527637822603658699938581184513";
        // const MODULUS: &'static str = "4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787";
        const MODULUS: &'static [u8] = &[
            171, 170, 255, 255, 255, 255, 254, 185, 255, 255, 83, 177, 254, 255, 171, 30, 36, 246,
            176, 246, 160, 210, 48, 103, 191, 18, 133, 243, 132, 75, 119, 100, 215, 172, 75, 67,
            182, 167, 27, 75, 154, 230, 127, 57, 234, 17, 1, 26,
        ];
        const G1_LEN: usize = 97;
        const G2_LEN: usize = 193;
        const SCALAR_LEN: usize = 32;
        const CURVE_ID: u32 = 0x10;
    }

    paste_test!(Bls12_381, Fq12);
}

mod bn254 {
    use super::*;
    use ark_bn254::{Bn254, Fq12, Fr, G1Projective, G2Projective};

    impl CurveBasicOperations for Bn254 {
        const SCALAR_FIELD: &'static str =
            "21888242871839275222246405745257275088548364400416034343698204186575808495617";
        // const MODULUS: &'static str = "21888242871839275222246405745257275088696311157297823662689037894645226208583";
        const MODULUS: &'static [u8] = &[
            71, 253, 124, 216, 22, 140, 32, 60, 141, 202, 113, 104, 145, 106, 129, 151, 93, 88,
            129, 129, 182, 69, 80, 184, 41, 160, 49, 225, 114, 78, 100, 48,
        ];
        const G1_LEN: usize = 65;
        const G2_LEN: usize = 129;
        const SCALAR_LEN: usize = 32;
        const CURVE_ID: u32 = 0x20;
    }

    paste_test!(Bn254, Fq12);
}

mod bw6_761 {
    use super::*;
    use ark_bw6_761::{Fq6, Fr, G1Projective, G2Projective, BW6_761};

    impl CurveBasicOperations for BW6_761 {
        const SCALAR_FIELD: &'static str = "6891450384315732539396789682275657542479668912536150109513790160209623422243491736087683183289411687640864567753786613451161759120554247759349511699125301598951605099378508850372543631423596795951899700429969112842764913119068299";
        // const MODULUS: &'static str = "6891450384315732539396789682275657542479668912536150109513790160209623422243491736087683183289411687640864567753786613451161759120554247759349511699125301598951605099378508850372543631423596795951899700429969112842764913119068299";
        const MODULUS: &'static [u8] = &[
            139, 0, 0, 0, 0, 0, 157, 244, 130, 0, 0, 112, 104, 62, 145, 230, 55, 164, 240, 234,
            174, 248, 12, 22, 248, 168, 103, 86, 194, 22, 161, 152, 46, 255, 235, 115, 220, 211,
            220, 113, 144, 253, 249, 18, 237, 200, 137, 134, 4, 35, 180, 37, 255, 186, 206, 3, 25,
            233, 132, 229, 56, 166, 123, 112, 65, 190, 135, 128, 239, 117, 130, 82, 136, 70, 209,
            129, 106, 24, 38, 185, 62, 255, 250, 4, 64, 201, 135, 209, 10, 206, 131, 251, 36, 232,
            34, 1,
        ];
        const G1_LEN: usize = 193;
        const G2_LEN: usize = 193;
        const SCALAR_LEN: usize = 48;
        const CURVE_ID: u32 = 0x30;
    }

    paste_test!(BW6_761, Fq6);
}

mod cp6_782 {
    use super::*;
    use ark_cp6_782::{Fq6, Fr, G1Projective, G2Projective, CP6_782};

    impl CurveBasicOperations for CP6_782 {
        const SCALAR_FIELD: &'static str = "22369874298875696930346742206501054934775599465297184582183496627646774052458024540232479018147881220178054575403841904557897715222633333372134756426301062487682326574958588001132586331462553235407484089304633076250782629492557320825577";
        // const MODULUS: &'static str = "22369874298875696930346742206501054934775599465297184582183496627646774052458024540232479018147881220178054575403841904557897715222633333372134756426301062487682326574958588001132586331462553235407484089304633076250782629492557320825577";
        const MODULUS: &'static [u8] = &[
            233, 42, 148, 123, 181, 121, 206, 218, 74, 66, 253, 109, 193, 133, 93, 84, 183, 38, 77,
            95, 6, 92, 19, 238, 75, 2, 196, 18, 74, 118, 47, 156, 57, 106, 254, 156, 4, 51, 213,
            26, 32, 19, 156, 199, 119, 251, 163, 82, 146, 87, 124, 97, 200, 150, 53, 171, 139, 215,
            249, 128, 141, 114, 12, 131, 7, 61, 2, 114, 238, 35, 114, 106, 38, 240, 106, 116, 107,
            23, 93, 188, 99, 102, 82, 143, 61, 40, 89, 233, 31, 148, 248, 171, 59, 38, 210, 196,
            72, 56,
        ];
        const G1_LEN: usize = 209;
        const G2_LEN: usize = 625;
        const SCALAR_LEN: usize = 48;
        const CURVE_ID: u32 = 0x4;
    }

    paste_test!(CP6_782, Fq6);
}
