use super::CurveBasicOperations;
use ark_bls12_381::Bls12_381;

impl CurveBasicOperations for Bls12_381 {
    const FQ_LEN: usize = 48;
    const SCALAR_LEN: usize = 32;
}
