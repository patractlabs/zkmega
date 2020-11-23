use crate::{parse, result::Result};
use bellman_ce::{
    groth16,
    pairing::{
        ff::{Field, PrimeField, PrimeFieldRepr, ScalarEngine},
        CurveAffine, CurveProjective, EncodedPoint, Engine, GroupDecodingError,
    },
};

/// Pairing-Friendly Curve
pub trait Curve: Engine + ScalarEngine {
    /// Add operation for all Pairing-Friendly Engines
    fn add(input: &[u8], output: &mut [u8]) -> Result<()> {
        let len = output.len();
        if input.len() != len * 2 {
            Err(GroupDecodingError::UnexpectedInformation.into())
        } else {
            let (p1, p2) = (
                parse::curve_affine::<<Self as Engine>::G1Affine>(&input[0..len]),
                parse::curve_affine::<<Self as Engine>::G1Affine>(&input[len..]),
            );

            // The added point
            let mut p = Self::G1::from(p1.into_affine()?);
            p.add_assign_mixed(&p2.into_affine()?);

            // Compose output stream
            output.copy_from_slice(p.into_affine().into_uncompressed().as_ref());
            Ok(())
        }
    }

    /// Mul operation for all Pairing-Friendly Engines
    fn mul(input: &[u8], output: &mut [u8]) -> Result<()>
    where
        <<Self as ScalarEngine>::Fr as PrimeField>::Repr: From<<Self as ScalarEngine>::Fr>,
    {
        let len = output.len();
        if input.len() != len + 32 {
            Err(GroupDecodingError::UnexpectedInformation.into())
        } else {
            let p1 = parse::curve_affine::<<Self as Engine>::G1Affine>(&input[0..len]);

            // Get scalar
            let m = <Self as ScalarEngine>::Fr::one();
            m.into_repr().write_be(&mut input[len..].to_vec()).unwrap();

            // Compose output stream
            let p = p1.into_affine()?.mul(m);
            output.copy_from_slice(p.into_affine().into_uncompressed().as_ref());
            Ok(())
        }
    }

    /// Pairing operation for Curves
    fn pairing(input: &[u8], g1_len: usize) -> Result<bool> {
        let element_len = g1_len * 3;
        if input.len() % element_len != 0 && !input.is_empty() {
            return Ok(false);
        }

        // Get pairs
        let mut pairs = Vec::new();
        for idx in 0..input.len() / element_len {
            let g1 = parse::curve_affine::<<Self as Engine>::G1Affine>(
                &input[idx * element_len..idx * element_len + 96],
            );
            let g2 = parse::curve_affine::<<Self as Engine>::G2Affine>(
                &input[(idx * element_len + 96)..(idx * element_len + 288)],
            );

            pairs.push((g1.into_affine()?.prepare(), g2.into_affine()?.prepare()))
        }

        // Check if pairing
        Ok(<Self as Engine>::final_exponentiation(&Self::miller_loop(
            &pairs.iter().map(|p| (&p.0, &p.1)).collect::<Vec<_>>(),
        )) == Some(<Self as Engine>::Fqk::one()))
    }

    /// Standard Verify
    fn verify<G1Affine, G2Affine, E>(
        alpha_g1: G1Affine,
        beta_g1: G1Affine,
        beta_g2: G2Affine,
        gamma_g2: G2Affine,
        delta_g1: G1Affine,
        delta_g2: G2Affine,
        ic: Vec<G1Affine>,
        proof_a: G1Affine,
        proof_b: G2Affine,
        proof_c: G1Affine,
        input: &[u64],
    ) -> Result<bool>
    where
        G1Affine: AsRef<[u8]>,
        G2Affine: AsRef<[u8]>,
        E: Engine + ScalarEngine,
    {
        Ok(groth16::verify_proof::<E>(
            &parse::verifying_key(alpha_g1, beta_g1, beta_g2, gamma_g2, delta_g1, delta_g2, ic)?,
            &parse::proof(proof_a, proof_b, proof_c)?,
            &parse::vector_fr::<E>(&input)?,
        )?)
    }
}

impl<T> Curve for T where T: Engine + ScalarEngine {}
