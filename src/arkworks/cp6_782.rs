use super::{all_curve_three_operations_test, CurveBasicOperations};
use ark_cp6_782::CP6_782;

impl CurveBasicOperations for CP6_782 {
    const G1_LEN: usize = 209;
    const G2_LEN: usize = 625;
    const SCALAR_LEN: usize = 48;
}

#[test]
fn test_cp6_782() {
    all_curve_three_operations_test::<CP6_782>();
}
