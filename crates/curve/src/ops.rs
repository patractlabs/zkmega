//! Curve Operations

pub use ark_ec::{AffineCurve, PairingEngine, ProjectiveCurve};
pub use ark_ff::{FromBytes, One, PrimeField, ToBytes, Zero};
pub use ark_serialize::SerializationError;
pub use ark_std::{
    io::{Error, ErrorKind},
    ops::{MulAssign, Neg},
    str::FromStr,
    test_rng,
    vec::Vec,
    UniformRand,
};

/// Arkworks Curve Ops
pub trait CurveBasicOperations: PairingEngine {
    // curve basic parameters
    const SCALAR_FIELD: &'static str;
    const MODULUS: &'static [u8];
    // G1 bytes length
    const G1_LEN: usize;
    // G2 bytes length
    const G2_LEN: usize;
    // Scalar bytes length
    const SCALAR_LEN: usize;
    // Curve ID
    const CURVE_ID: u32;

    fn add(input: &[u8]) -> Result<Vec<u8>, SerializationError> {
        // g1 infinity is bool, so two g1s should be + 2 byte.
        if input.len() != Self::G1_LEN * 2 {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "add operation input invalid length, should be {}, \
                     input length: {:?}",
                    Self::G1_LEN * 2,
                    input.len(),
                ),
            )
            .into());
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

    fn mul(input: &[u8]) -> Result<Vec<u8>, SerializationError> {
        // g1 infinity is bool, so + 1 byte.
        if input.len() != Self::G1_LEN + Self::SCALAR_LEN {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "scalar_mul operation input invalid length, should be {}, \
                     input length: {:?}",
                    Self::G1_LEN + Self::SCALAR_LEN,
                    input.len(),
                ),
            )
            .into());
        }
        let point =
            <<Self as PairingEngine>::G1Affine as FromBytes>::read(&input[0..Self::G1_LEN])?;
        let scalar = <<Self as PairingEngine>::Fr as FromBytes>::read(&input[Self::G1_LEN..])?;

        let mul_res = point.into_projective().mul(scalar.into_repr());
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
                format!(
                    "pairing operation input invalid length, should be {} \
                     input length: {:?}",
                    g1_g2_len,
                    input.len(),
                ),
            )
            .into());
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
