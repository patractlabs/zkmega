use super::{all_curve_three_operations_test, test_pairings, CurveBasicOperations};
use ark_bn254::{Bn254, Fq12, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{AffineCurve, PairingEngine, ProjectiveCurve};
use ark_ff::{test_rng, One, ToBytes, Zero};
use ark_std::str::FromStr;
use ark_std::{
    ops::{MulAssign, Neg},
    vec::Vec,
};

impl CurveBasicOperations for Bn254 {
    const G1_LEN: usize = 65;
    const G2_LEN: usize = 129;
    const SCALAR_LEN: usize = 32;
}

#[test]
fn test_bn256() {
    // all_curve_three_operations_test::<Bn254>();
    test_pairings::<Bn254>();
}
