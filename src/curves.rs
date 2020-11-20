use pairing_ce::{
    ff::{Field, PrimeField, PrimeFieldRepr, ScalarEngine},
    CurveAffine, CurveProjective, EncodedPoint, Engine, GroupDecodingError,
};

/// Pairing-Friendly Curve
pub trait Curve<E: Engine, S: ScalarEngine> {
    /// Add operation for all Pairing-Friendly Engines
    fn add(input: &[u8], output: &mut [u8]) -> Result<(), GroupDecodingError> {
        let len = output.len();
        if input.len() != len * 2 {
            Err(GroupDecodingError::UnexpectedInformation)
        } else {
            let (mut p1, mut p2) = (
                <<<E as Engine>::G1Affine as CurveAffine>::Uncompressed as EncodedPoint>::empty(),
                <<<E as Engine>::G1Affine as CurveAffine>::Uncompressed as EncodedPoint>::empty(),
            );
            p1.as_mut().copy_from_slice(&input[0..len]);
            p2.as_mut().copy_from_slice(&input[len..]);

            // The added point
            let mut p = E::G1::from(p1.into_affine()?);
            p.add_assign_mixed(&p2.into_affine()?);

            // Compose output stream
            output.copy_from_slice(p.into_affine().into_uncompressed().as_ref());
            Ok(())
        }
    }

    /// Mul operation for all Pairing-Friendly Engines
    fn mul(input: &[u8], output: &mut [u8]) -> Result<(), GroupDecodingError>
    where
        <<E as ScalarEngine>::Fr as PrimeField>::Repr: From<<S as ScalarEngine>::Fr>,
    {
        let len = output.len();
        if input.len() != len + 32 {
            Err(GroupDecodingError::UnexpectedInformation)
        } else {
            let mut p1 =
                <<<E as Engine>::G1Affine as CurveAffine>::Uncompressed as EncodedPoint>::empty();
            p1.as_mut().copy_from_slice(&input[0..len]);

            // Get scalar
            let m = <S as ScalarEngine>::Fr::one();
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
        if input.len() % element_len != 0 && input.len() != 0 {
            return Ok(false);
        }

        // Get pairs
        let mut pairs = Vec::new();
        for idx in 0..input.len() / element_len {
            let mut g1 =
                <<<E as Engine>::G1Affine as CurveAffine>::Uncompressed as EncodedPoint>::empty();
            let mut g2 =
                <<<E as Engine>::G2Affine as CurveAffine>::Uncompressed as EncodedPoint>::empty();
            g1.as_mut()
                .copy_from_slice(&input[idx * element_len..idx * element_len + 96]);
            g2.as_mut()
                .copy_from_slice(&input[(idx * element_len + 96)..(idx * element_len + 288)]);

            pairs.push((g1.into_affine()?.prepare(), g2.into_affine()?.prepare()))
        }

        // Check if pairing
        Ok(
            <E as pairing_ce::Engine>::final_exponentiation(&E::miller_loop(
                &pairs.iter().map(|p| (&p.0, &p.1)).collect::<Vec<_>>(),
            )) == Some(<E as Engine>::Fqk::one()),
        )
    }
}
