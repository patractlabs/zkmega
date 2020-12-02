use super::{all_curve_three_operations_test, CurveBasicOperations};
use ark_bw6_761::BW6_761;

impl CurveBasicOperations for BW6_761 {
    const G1_LEN: usize = 193;
    const G2_LEN: usize = 193;
    const SCALAR_LEN: usize = 48;
}

#[test]
fn test_bw6_761() {
    all_curve_three_operations_test::<BW6_761>();
}
