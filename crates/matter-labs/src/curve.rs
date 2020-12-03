//! Matter Lab Curves
use crate::result::Result;
use bellman_ce::pairing::{
    ff::{Field, PrimeField, PrimeFieldRepr, ScalarEngine},
    CurveAffine, CurveProjective, EncodedPoint, Engine, GroupDecodingError,
};

/// Custom bytes
pub trait Bytes: AsRef<[u8]> + AsMut<[u8]> {
    fn default() -> Self;
}

/// Generate G point from bytes
pub fn curve_affine<C: CurveAffine>(input: &[u8]) -> C::Uncompressed {
    let mut p = <C::Uncompressed as EncodedPoint>::empty();
    p.as_mut().copy_from_slice(&input);
    p
}

/// Pairing-Friendly Curve
pub trait Curve: Engine + ScalarEngine {
    type Output: Bytes;

    /// Add operation for all Pairing-Friendly Engines
    fn add(input: &[u8]) -> Result<Self::Output> {
        let mut output = <Self::Output as Bytes>::default();
        let len = output.as_ref().len();
        if input.len() != len * 2 {
            Err(GroupDecodingError::UnexpectedInformation.into())
        } else {
            let (p1, p2) = (
                curve_affine::<<Self as Engine>::G1Affine>(&input[0..len]),
                curve_affine::<<Self as Engine>::G1Affine>(&input[len..]),
            );

            // The added point
            let mut p = Self::G1::from(p1.into_affine()?);
            p.add_assign_mixed(&p2.into_affine()?);

            // Compose output stream
            output
                .as_mut()
                .copy_from_slice(p.into_affine().into_uncompressed().as_ref());
            Ok(output)
        }
    }

    /// Mul operation for all Pairing-Friendly Engines
    fn mul(input: &[u8]) -> Result<Self::Output> {
        let mut output = <Self::Output as Bytes>::default();
        let len = output.as_ref().len();
        if input.len() != len + 32 {
            Err(GroupDecodingError::UnexpectedInformation.into())
        } else {
            let p1 = curve_affine::<<Self as Engine>::G1Affine>(&input[0..len]);

            // Get scalar
            let m = <Self as ScalarEngine>::Fr::one();
            m.into_repr().read_be(&input[len..]).unwrap();

            // Compose output stream
            let p = p1.into_affine()?.mul(m);
            output
                .as_mut()
                .copy_from_slice(p.into_affine().into_uncompressed().as_ref());
            Ok(output)
        }
    }

    /// Pairing operation for Curves
    fn pairing(input: &[u8]) -> Result<bool> {
        let g1_len = <Self::Output as Bytes>::default().as_ref().len();
        let element_len = g1_len * 3;
        if input.len() % element_len != 0 && !input.is_empty() {
            return Ok(false);
        }

        // Get pairs
        let mut pairs = Vec::new();
        for idx in 0..input.len() / element_len {
            let g1 = curve_affine::<<Self as Engine>::G1Affine>(
                &input[idx * element_len..idx * element_len + g1_len],
            );
            let g2 = curve_affine::<<Self as Engine>::G2Affine>(
                &input[(idx * element_len + g1_len)..(idx * element_len + element_len)],
            );

            pairs.push((g1.into_affine()?.prepare(), g2.into_affine()?.prepare()))
        }

        // Check if pairing
        Ok(<Self as Engine>::final_exponentiation(&Self::miller_loop(
            &pairs.iter().map(|p| (&p.0, &p.1)).collect::<Vec<_>>(),
        )) == Some(<Self as Engine>::Fqk::one()))
    }
}
