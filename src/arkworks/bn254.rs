use super::{test_pairings, CurveBasicOperations};
use ark_bn254::Bn254;

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
