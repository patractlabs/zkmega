use super::CurveBasicOperations;
use ark_cp6_782::{Fq6, Fr, G1Projective, G2Projective, CP6_782};
use ark_ec::PairingEngine;
use ark_ff::{test_rng, Field, One, PrimeField};
use ark_std::{ops::MulAssign, vec::Vec};
use rand::Rng;
use rustc_hex::FromHex;

impl CurveBasicOperations for CP6_782 {
    const G1_LEN: usize = 209;
    const G2_LEN: usize = 625;
    const SCALAR_LEN: usize = 48;
    const CURVE_ID: usize = 0x2d;
}

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

    let ans1 = CP6_782::pairing(sa, b);
    let ans2 = CP6_782::pairing(a, sb);
    let ans3 = CP6_782::pairing(a, b).pow(s.into_repr());

    assert_eq!(ans1, ans2);
    assert_eq!(ans2, ans3);

    assert_ne!(ans1, Fq6::one());
    assert_ne!(ans2, Fq6::one());
    assert_ne!(ans3, Fq6::one());

    assert_eq!(ans1.pow(Fr::characteristic()), Fq6::one());
    assert_eq!(ans2.pow(Fr::characteristic()), Fq6::one());
    assert_eq!(ans3.pow(Fr::characteristic()), Fq6::one());
}
