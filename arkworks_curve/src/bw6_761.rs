use super::{all_curve_three_operations_test, CurveBasicOperations};
use ark_bw6_761::{Fq6, Fr, G1Projective, G2Projective, BW6_761};
use ark_ec::PairingEngine;
use ark_ff::{test_rng, Field, One, PrimeField};
use ark_std::ops::MulAssign;
use rand::Rng;
use rustc_hex::FromHex;

impl CurveBasicOperations for BW6_761 {
    const G1_LEN: usize = 193;
    const G2_LEN: usize = 193;
    const SCALAR_LEN: usize = 48;
}

#[test]
fn test_bw6_761() {
    all_curve_three_operations_test::<BW6_761>();
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

    let ans1 = BW6_761::pairing(sa, b);
    let ans2 = BW6_761::pairing(a, sb);
    let ans3 = BW6_761::pairing(a, b).pow(s.into_repr());

    assert_eq!(ans1, ans2);
    assert_eq!(ans2, ans3);

    assert_ne!(ans1, Fq6::one());
    assert_ne!(ans2, Fq6::one());
    assert_ne!(ans3, Fq6::one());

    assert_eq!(ans1.pow(Fr::characteristic()), Fq6::one());
    assert_eq!(ans2.pow(Fr::characteristic()), Fq6::one());
    assert_eq!(ans3.pow(Fr::characteristic()), Fq6::one());
}
