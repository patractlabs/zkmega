use super::{test_pairings, CurveBasicOperations};
use ark_bls12_377::Bls12_377;

impl CurveBasicOperations for Bls12_377 {
    const G1_LEN: usize = 97;
    const G2_LEN: usize = 193;
    const SCALAR_LEN: usize = 32;
}

#[test]
fn test_bls12_377() {
    // all_curve_three_operations_test::<Bls12_377>();
    test_pairings::<Bls12_377>();
}
