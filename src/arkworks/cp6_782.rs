use super::{test_pairings, CurveBasicOperations};
use ark_cp6_782::CP6_782;

impl CurveBasicOperations for CP6_782 {
    const G1_LEN: usize = 209;
    const G2_LEN: usize = 625;
    const SCALAR_LEN: usize = 48;
}

#[test]
fn test_cp6_782() {
    test_pairings::<CP6_782>();
}
