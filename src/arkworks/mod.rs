mod bls12_377;
mod bls12_381;
mod bn254;
mod bw6_761;
mod cp6_782;

use ark_ec::{AffineCurve, PairingEngine, ProjectiveCurve};
use ark_ff::{FromBytes, One, ToBytes};
use ark_serialize::{
    CanonicalDeserialize, CanonicalSerialize, ConstantSerializedSize, SerializationError,
};
use ark_std::io::{Cursor, Error, ErrorKind, Write};

trait CurveBasicOperations: PairingEngine {
    // Fq bytes length
    const FQ_LEN: usize;
    // Scalar bytes length
    const SCALAR_LEN: usize;

    fn add(input: &[u8]) -> Result<Vec<u8>, SerializationError> {
        // g1 infinity is bool, so two g1s should be + 2 byte.
        if input.len() != Self::FQ_LEN * 4 + 2 {
            return Err(Error::new(
                ErrorKind::Other,
                "add operation input invalid length",
            ))?;
        }
        let point1 = <<Self as PairingEngine>::G1Affine as FromBytes>::read(
            &input[0..Self::FQ_LEN * 2 + 1],
        )?;
        let point2 =
            <<Self as PairingEngine>::G1Affine as FromBytes>::read(&input[Self::FQ_LEN * 2 + 1..])?;

        let sum_res = point1 + point2;
        let mut output = Vec::new();

        sum_res.write(&mut output);
        Ok(output)
    }

    fn scalar_mul(input: &[u8]) -> Result<Vec<u8>, SerializationError> {
        // g1 infinity is bool, so + 1 byte.
        if input.len() != Self::FQ_LEN * 2 + 1 + Self::SCALAR_LEN {
            return Err(Error::new(
                ErrorKind::Other,
                "scalar_mul operation input invalid length",
            ))?;
        }
        let mut point = <<Self as PairingEngine>::G1Affine as FromBytes>::read(
            &input[0..Self::FQ_LEN * 2 + 1],
        )?;
        let scalar =
            <<Self as PairingEngine>::Fr as FromBytes>::read(&input[Self::FQ_LEN * 2 + 1..])?;

        let mul_res = point.into_projective().mul(scalar);
        let mut output = Vec::new();
        mul_res.into_affine().write(&mut output);
        Ok(output)
    }

    fn pairings(input: &[u8]) -> Result<bool, SerializationError> {
        // g1 infinity is bool, so + 1 byte.
        let g1_len = Self::FQ_LEN * 2 + 1;
        // ditto, g1 g2 + 2.
        let g1_g2_len = Self::FQ_LEN * 6 + 2;
        if input.len() % g1_g2_len != 0 && !input.is_empty() {
            return Err(Error::new(
                ErrorKind::Other,
                "pairing operation input invalid length",
            ))?;
        }

        // Get pairs
        let mut pairings = Vec::new();
        for i in 0..input.len() / g1_g2_len {
            let g1 = <<Self as PairingEngine>::G1Affine as FromBytes>::read(
                &input[i * g1_g2_len..i * g1_g2_len + g1_len],
            )?;
            let g2 = <<Self as PairingEngine>::G2Affine as FromBytes>::read(
                &input[i * g1_g2_len + g1_len..(i + 1) * g1_g2_len],
            )?;

            pairings.push((g1.into(), g2.into()))
        }
        assert_eq!(
            <Self as PairingEngine>::product_of_pairings(&pairings),
            <Self as PairingEngine>::Fqk::one()
        );
        // Check if pairing
        Ok(<Self as PairingEngine>::product_of_pairings(&pairings)
            == <Self as PairingEngine>::Fqk::one())
    }
}
