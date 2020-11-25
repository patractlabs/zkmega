#![macro_use]
use crate::{parse, result::Result};
use bellman_ce::{
    groth16,
    pairing::{
        ff::{Field, PrimeField, PrimeFieldRepr, ScalarEngine},
        CurveAffine, CurveProjective, EncodedPoint, Engine, GroupDecodingError,
    },
};

/// Custom bytes
pub trait Bytes: AsRef<[u8]> + AsMut<[u8]> {
    fn default() -> Self;
}

/// Pairing-Friendly Curve
pub trait Curve: Engine + ScalarEngine {
    /// Add operation for all Pairing-Friendly Engines
    fn add<Output>(input: &[u8]) -> Result<Output>
    where
        Output: Bytes,
    {
        let mut output = <Output as Bytes>::default();
        let len = output.as_ref().len();
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
            output
                .as_mut()
                .copy_from_slice(p.into_affine().into_uncompressed().as_ref());
            Ok(output)
        }
    }

    /// Mul operation for all Pairing-Friendly Engines
    fn mul<Output>(input: &[u8]) -> Result<Output>
    where
        <<Self as ScalarEngine>::Fr as PrimeField>::Repr: From<<Self as ScalarEngine>::Fr>,
        Output: Bytes,
    {
        let mut output = <Output as Bytes>::default();
        let len = output.as_ref().len();
        if input.len() != len + 32 {
            Err(GroupDecodingError::UnexpectedInformation.into())
        } else {
            let p1 = parse::curve_affine::<<Self as Engine>::G1Affine>(&input[0..len]);

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
    fn pairing<Output>(input: &[u8]) -> Result<bool>
    where
        Output: Bytes,
    {
        let g1_len = <Output as Bytes>::default().as_ref().len();
        let element_len = g1_len * 3;
        if input.len() % element_len != 0 && !input.is_empty() {
            return Ok(false);
        }

        // Get pairs
        let mut pairs = Vec::new();
        for idx in 0..input.len() / element_len {
            let g1 = parse::curve_affine::<<Self as Engine>::G1Affine>(
                &input[idx * element_len..idx * element_len + g1_len],
            );
            let g2 = parse::curve_affine::<<Self as Engine>::G2Affine>(
                &input[(idx * element_len + g1_len)..(idx * element_len + element_len)],
            );

            pairs.push((g1.into_affine()?.prepare(), g2.into_affine()?.prepare()))
        }

        // Check if pairing
        Ok(<Self as Engine>::final_exponentiation(&Self::miller_loop(
            &pairs.iter().map(|p| (&p.0, &p.1)).collect::<Vec<_>>(),
        )) == Some(<Self as Engine>::Fqk::one()))
    }

    /// Standard Verify
    fn verify<G1Affine, G2Affine>(
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
    {
        Ok(groth16::verify_proof::<Self>(
            &parse::verifying_key(alpha_g1, beta_g1, beta_g2, gamma_g2, delta_g1, delta_g2, ic)?,
            &parse::proof(proof_a, proof_b, proof_c)?,
            &parse::vector_fr::<Self>(&input)?,
        )?)
    }
}

impl<T> Curve for T where T: Engine + ScalarEngine {}

/// Declare curve
#[macro_export]
macro_rules! curve {
    ($curve:ident, $g1:expr, $g2:expr) => {
        /// Op Bytes
        type OpOutput = [u8; $g1];
        impl Bytes for OpOutput {
            fn default() -> [u8; $g1] {
                [0; $g1]
            }
        }

        /// bls12_381 add
        pub fn add(input: &[u8]) -> Result<OpOutput> {
            <$curve as Curve>::add::<OpOutput>(input)
        }

        /// bls12_381 mul
        pub fn mul(input: &[u8]) -> Result<OpOutput> {
            <$curve as Curve>::mul::<OpOutput>(input)
        }

        /// bls12_381 pairing
        pub fn pairing(input: &[u8]) -> Result<bool> {
            <$curve as Curve>::pairing::<OpOutput>(input)
        }

        ///! bls12_381  verify
        pub fn verify(
            alpha_g1: [u8; $g1],
            beta_g1: [u8; $g1],
            beta_g2: [u8; $g2],
            gamma_g2: [u8; $g2],
            delta_g1: [u8; $g1],
            delta_g2: [u8; $g2],
            ic: Vec<[u8; $g1]>,
            proof_a: [u8; $g1],
            proof_b: [u8; $g2],
            proof_c: [u8; $g1],
            input: &[u64],
        ) -> Result<bool> {
            <$curve as Curve>::verify::<[u8; $g1], [u8; $g2]>(
                alpha_g1, beta_g1, beta_g2, gamma_g2, delta_g1, delta_g2, ic, proof_a, proof_b,
                proof_c, input,
            )
        }
    };
}
