use super::{all_curve_three_operations_test, test_pairings, CurveBasicOperations};
use ark_bls12_381::Bls12_381;

impl CurveBasicOperations for Bls12_381 {
    const G1_LEN: usize = 97;
    const G2_LEN: usize = 193;
    const SCALAR_LEN: usize = 32;
}

#[test]
fn test_bls12_381() {
    // all_curve_three_operations_test::<Bls12_381>();
    test_pairings::<Bls12_381>();
}
