use super::all_curve_three_operations_test;
use arkworks::curves::{Bls12_377, Bls12_381, Bn254, BW6_761, CP6_782};

#[test]
fn test_bls12_381() {
    all_curve_three_operations_test::<Bls12_381>();
}

#[test]
fn test_bls12_377() {
    all_curve_three_operations_test::<Bls12_377>();
}

#[test]
fn test_bn254() {
    all_curve_three_operations_test::<Bn254>();
}

#[test]
fn test_bw6_761() {
    all_curve_three_operations_test::<BW6_761>();
}

#[test]
fn test_cp6_782() {
    all_curve_three_operations_test::<CP6_782>();
}
