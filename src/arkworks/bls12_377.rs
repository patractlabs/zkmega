use super::CurveBasicOperations;
use ark_bls12_377::Bls12_377;

impl CurveBasicOperations for Bls12_377 {
    const FQ_LEN: usize = 0;
    const SCALAR_LEN: usize = 0;
}
