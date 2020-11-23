//! Util functions
use bellman_ce::{
    groth16::{prepare_verifying_key, PreparedVerifyingKey, Proof, VerifyingKey},
    pairing::{CurveAffine, EncodedPoint, Engine, GroupDecodingError},
};

/// Generate G point from bytes
pub fn curve_affine<C: CurveAffine>(input: &[u8]) -> C::Uncompressed {
    let mut p = <C::Uncompressed as EncodedPoint>::empty();
    p.as_mut().copy_from_slice(&input);
    p
}

/// Parse proof
pub fn proof<G1Affine, G2Affine, E>(
    a: G1Affine,
    b: G2Affine,
    c: G1Affine,
) -> Result<Proof<E>, GroupDecodingError>
where
    G1Affine: AsRef<[u8]>,
    G2Affine: AsRef<[u8]>,
    E: Engine,
{
    Ok(Proof {
        a: curve_affine::<E::G1Affine>(a.as_ref()).into_affine()?,
        b: curve_affine::<E::G2Affine>(b.as_ref()).into_affine()?,
        c: curve_affine::<E::G1Affine>(c.as_ref()).into_affine()?,
    })
}

/// Generate VerifyingKey from bytes
pub fn verifying_key<G1Affine, G2Affine, E>(
    alpha_g1: G1Affine,
    beta_g1: G1Affine,
    beta_g2: G2Affine,
    gamma_g2: G2Affine,
    delta_g1: G1Affine,
    delta_g2: G2Affine,
    ic: Vec<G1Affine>,
) -> Result<PreparedVerifyingKey<E>, GroupDecodingError>
where
    G1Affine: AsRef<[u8]>,
    G2Affine: AsRef<[u8]>,
    E: Engine,
{
    let mut icv = vec![];
    for i in ic {
        icv.push(curve_affine::<E::G1Affine>(i.as_ref()).into_affine()?);
    }

    Ok(prepare_verifying_key::<E>(&VerifyingKey {
        alpha_g1: curve_affine::<E::G1Affine>(alpha_g1.as_ref()).into_affine()?,
        beta_g1: curve_affine::<E::G1Affine>(beta_g1.as_ref()).into_affine()?,
        beta_g2: curve_affine::<E::G2Affine>(beta_g2.as_ref()).into_affine()?,
        gamma_g2: curve_affine::<E::G2Affine>(gamma_g2.as_ref()).into_affine()?,
        delta_g1: curve_affine::<E::G1Affine>(delta_g1.as_ref()).into_affine()?,
        delta_g2: curve_affine::<E::G2Affine>(delta_g2.as_ref()).into_affine()?,
        ic: icv,
    }))
}
