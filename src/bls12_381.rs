use crate::{result::Result, scratch::Curve};
use bellman_ce::pairing::bls12_381::Bls12;

const G1_LENGTH: usize = 64;

/// bls12_381 add
pub fn add(input: &[u8], output: &mut [u8]) -> Result<()> {
    <Bls12 as Curve>::add(input, output)
}

/// bls12_381 mul
pub fn mul(input: &[u8], output: &mut [u8]) -> Result<()> {
    <Bls12 as Curve>::mul(input, output)
}

/// bls12_381 pairing
pub fn pairing(input: &[u8]) -> Result<bool> {
    <Bls12 as Curve>::pairing(input, G1_LENGTH)
}
