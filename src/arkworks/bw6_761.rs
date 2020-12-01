use super::CurveBasicOperations;
use ark_bw6_761::BW6_761;

impl CurveBasicOperations for BW6_761 {
    const FQ_LEN: usize = 0;
    const SCALAR_LEN: usize = 0;
}
