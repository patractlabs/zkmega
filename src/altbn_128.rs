use crate::{result::Result, scratch::Curve};
use bellman_ce::pairing::bn256::Bn256;

/// The G1 length of altbn_128
const G1_LENGTH: usize = 96;

/// altbn_128 add
pub fn add(input: &[u8], output: &mut [u8]) -> Result<()> {
    <Bn256 as Curve>::add(input, output)
}

/// altbn_128 mul
pub fn mul(input: &[u8], output: &mut [u8]) -> Result<()> {
    <Bn256 as Curve>::mul(input, output)
}

/// altbn_128 pairing
pub fn pairing(input: &[u8]) -> Result<bool> {
    <Bn256 as Curve>::pairing(input, G1_LENGTH)
}
