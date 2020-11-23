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

///! bls12_381  verify
pub fn verify(
    alpha_g1: [u8; 96],
    beta_g1: [u8; 96],
    beta_g2: [u8; 192],
    gamma_g2: [u8; 192],
    delta_g1: [u8; 96],
    delta_g2: [u8; 192],
    ic: Vec<[u8; 96]>,
    proof_a: [u8; 96],
    proof_b: [u8; 192],
    proof_c: [u8; 96],
    input: &[u64],
) -> Result<bool> {
    <Bls12 as Curve>::verify::<[u8; 96], [u8; 192]>(
        alpha_g1, beta_g1, beta_g2, gamma_g2, delta_g1, delta_g2, ic, proof_a, proof_b, proof_c,
        input,
    )
}
