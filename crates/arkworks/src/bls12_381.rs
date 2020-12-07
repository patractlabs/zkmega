use super::CurveBasicOperations;
use ark_bls12_381::{Bls12_381, Fq12, Fr, G1Projective, G2Projective};
use ark_ec::PairingEngine;
use ark_ff::{test_rng, Field, One, PrimeField};
use ark_std::{ops::MulAssign, vec::Vec};
use rand::Rng;
use rustc_hex::FromHex;

impl CurveBasicOperations for Bls12_381 {
    const G1_LEN: usize = 97;
    const G2_LEN: usize = 193;
    const SCALAR_LEN: usize = 32;
    const CURVE_ID: usize = 0x2b;
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

    let ans1 = Bls12_381::pairing(sa, b);
    let ans2 = Bls12_381::pairing(a, sb);
    let ans3 = Bls12_381::pairing(a, b).pow(s.into_repr());

    assert_eq!(ans1, ans2);
    assert_eq!(ans2, ans3);

    assert_ne!(ans1, Fq12::one());
    assert_ne!(ans2, Fq12::one());
    assert_ne!(ans3, Fq12::one());

    assert_eq!(ans1.pow(Fr::characteristic()), Fq12::one());
    assert_eq!(ans2.pow(Fr::characteristic()), Fq12::one());
    assert_eq!(ans3.pow(Fr::characteristic()), Fq12::one());
}
