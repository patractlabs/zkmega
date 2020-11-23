use crate::parse;
use bellman_ce::pairing::{
    ff::{Field, PrimeField, PrimeFieldRepr, ScalarEngine},
    CurveAffine, CurveProjective, EncodedPoint, Engine, GroupDecodingError,
};

/// Pairing-Friendly Curve
pub trait Curve: Engine + ScalarEngine {
    /// Add operation for all Pairing-Friendly Engines
    fn add(input: &[u8], output: &mut [u8]) -> Result<(), GroupDecodingError> {
        let len = output.len();
        if input.len() != len * 2 {
            Err(GroupDecodingError::UnexpectedInformation)
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
    fn mul(input: &[u8], output: &mut [u8]) -> Result<(), GroupDecodingError>
    where
        <<Self as ScalarEngine>::Fr as PrimeField>::Repr: From<<Self as ScalarEngine>::Fr>,
    {
        let len = output.len();
        if input.len() != len + 32 {
            Err(GroupDecodingError::UnexpectedInformation)
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
    fn pairing(input: &[u8], g1_len: usize) -> Result<bool, GroupDecodingError> {
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
}

impl<T> Curve for T where T: Engine + ScalarEngine {}
