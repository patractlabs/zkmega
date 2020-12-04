#![allow(unused_doc_comments)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(non_snake_case)]
#![no_std]
mod bls12_377;
mod bls12_381;
mod bn254;
mod bw6_761;
mod cp6_782;
pub mod tests;

use ark_ec::{AffineCurve, PairingEngine, ProjectiveCurve};
use ark_ff::{test_rng, FromBytes, One, PrimeField, ToBytes, UniformRand, Zero};
use ark_serialize::SerializationError;
use ark_std::{
    io::{Error, ErrorKind},
    ops::{MulAssign, Neg},
    str::FromStr,
    vec::Vec,
};

/// Re-export curves
pub mod curves {
    pub use ark_bls12_377::Bls12_377;
    pub use ark_bls12_381::Bls12_381;
    pub use ark_bn254::Bn254;
    pub use ark_bw6_761::BW6_761;
    pub use ark_cp6_782::CP6_782;
}

pub trait CurveBasicOperations: PairingEngine {
    // G1 bytes length
    const G1_LEN: usize;
    // G2 bytes length
    const G2_LEN: usize;
    // Scalar bytes length
    const SCALAR_LEN: usize;

    fn add(input: &[u8]) -> Result<Vec<u8>, SerializationError> {
        // g1 infinity is bool, so two g1s should be + 2 byte.
        if input.len() != Self::G1_LEN * 2 {
            return Err(Error::new(
                ErrorKind::Other,
                "add operation input invalid length",
            ))?;
        }
        let point1 =
            <<Self as PairingEngine>::G1Affine as FromBytes>::read(&input[0..Self::G1_LEN])?;
        let point2 =
            <<Self as PairingEngine>::G1Affine as FromBytes>::read(&input[Self::G1_LEN..])?;

        let sum_res = point1 + point2;
        let mut output = Vec::new();

        sum_res.write(&mut output)?;
        Ok(output)
    }

    fn scalar_mul(input: &[u8]) -> Result<Vec<u8>, SerializationError> {
        // g1 infinity is bool, so + 1 byte.
        if input.len() != Self::G1_LEN + Self::SCALAR_LEN {
            return Err(Error::new(
                ErrorKind::Other,
                "scalar_mul operation input invalid length",
            ))?;
        }
        let point =
            <<Self as PairingEngine>::G1Affine as FromBytes>::read(&input[0..Self::G1_LEN])?;
        let scalar = <<Self as PairingEngine>::Fr as FromBytes>::read(&input[Self::G1_LEN..])?;

        let mul_res = point.into_projective().mul(scalar);
        let mut output = Vec::new();
        mul_res.into_affine().write(&mut output)?;
        Ok(output)
    }

    fn pairings(input: &[u8]) -> Result<bool, SerializationError> {
        // g1 infinity is bool, so + 1 byte.
        let g1_len = Self::G1_LEN;
        // ditto, g1 g2 + 2.
        let g1_g2_len = Self::G1_LEN + Self::G2_LEN;
        if input.len() % g1_g2_len != 0 && !input.is_empty() {
            return Err(Error::new(
                ErrorKind::Other,
                "pairing operation input invalid length",
            ))?;
        }

        // Get pairs
        let mut pairings = Vec::with_capacity(4);
        for i in 0..input.len() / g1_g2_len {
            let g1 = <<Self as PairingEngine>::G1Affine as FromBytes>::read(
                &input[i * g1_g2_len..i * g1_g2_len + g1_len],
            )?;
            let g2 = <<Self as PairingEngine>::G2Affine as FromBytes>::read(
                &input[i * g1_g2_len + g1_len..(i + 1) * g1_g2_len],
            )?;

            pairings.push((g1.into(), g2.into()))
        }

        // Check if pairing
        Ok(<Self as PairingEngine>::product_of_pairings(&pairings)
            == <Self as PairingEngine>::Fqk::one())
    }
}
