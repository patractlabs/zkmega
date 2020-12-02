mod bls12_377;
mod bls12_381;
mod bn254;
mod bw6_761;
mod cp6_782;

use ark_ec::{AffineCurve, PairingEngine, ProjectiveCurve};
use ark_ff::{test_rng, FromBytes, One, PrimeField, ToBytes, UniformRand, Zero};
use ark_serialize::SerializationError;
use ark_std::{
    io::{Error, ErrorKind},
    ops::{MulAssign, Neg},
    str::FromStr,
    vec::Vec,
};

pub trait CurveBasicOperations: PairingEngine {
    // G1 bytes length
    const G1_LEN: usize;
    // G2 bytes length
    const G2_LEN: usize;
    // Scalar bytes length
    const SCALAR_LEN: usize;

    fn add(input: &[u8]) -> Result<Vec<u8>, SerializationError> {
        // g1 infinity is bool, so two g1s should be + 2 byte.
        println!("{}", input.len());
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

        sum_res.write(&mut output);
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
        mul_res.into_affine().write(&mut output);
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

type G1Affine<T> = <T as PairingEngine>::G1Affine;
type G2Affine<T> = <T as PairingEngine>::G2Affine;
type G1Projective<T> = <T as PairingEngine>::G1Projective;
type G2Projective<T> = <T as PairingEngine>::G2Projective;
type Fr<T> = <T as PairingEngine>::Fr;

/// Test operations for all curves
pub fn all_curve_three_operations_test<T>()
where
    T: CurveBasicOperations + PairingEngine,
{
    // zero-points additions
    {
        let mut input = Vec::new();
        <G1Affine<T> as Zero>::zero().write(&mut input);
        <G1Affine<T> as Zero>::zero().write(&mut input);

        let mut expected = Vec::new();
        <G1Affine<T> as Zero>::zero().write(&mut expected);

        let res = T::add(&input[..]).unwrap();
        assert_eq!(&expected[..], &res[..]);
        println!("test add1 success!");
    }

    // one-points additions
    {
        let mut input1 = Vec::new();
        <G1Affine<T> as AffineCurve>::prime_subgroup_generator().write(&mut input1);
        <G1Affine<T> as AffineCurve>::prime_subgroup_generator().write(&mut input1);

        let mut input2 = Vec::new();
        <G1Affine<T> as AffineCurve>::prime_subgroup_generator().write(&mut input2);
        <Fr<T> as PrimeField>::from_repr(2u64.into()).map(|x| x.write(&mut input2));

        let res1 = T::add(&input1[..]).unwrap();
        let res2 = T::scalar_mul(&input2[..]).unwrap();
        assert_eq!(res1, res2);
        println!("test add2 success!");
    }

    // Prime subgroup generator additions check prime subgroup generator * 2(scalar_mul)
    {
        let mut input1 = Vec::new();
        <G1Affine<T> as AffineCurve>::prime_subgroup_generator().write(&mut input1);

        let mut input2 = Vec::new();
        <G1Affine<T> as AffineCurve>::prime_subgroup_generator().write(&mut input2);
        <Fr<T> as FromStr>::from_str("2").map(|x| x.write(&mut input2));

        let res1 = T::add(&input1.repeat(2)[..]).expect("Generator add failed");
        let res2 = T::scalar_mul(&input2[..]).expect("Generator scalar_mul 2 failed");

        let mut res3 = Vec::new();
        <G1Affine<T> as AffineCurve>::prime_subgroup_generator()
            .into_projective()
            .double()
            .into_affine()
            .write(&mut res3);

        // prime_subgroup_generator + prime_subgroup_generator = prime_subgroup_generator * 2
        assert_eq!(res1, res3);
        println!("test add3 success!");
        assert_eq!(res2, res3);
        println!("test scalar_mul1 success!");
    }

    // test pairings
    {
        for i in 0..10 {
            let mut rng = test_rng();
            let a = <G1Projective<T> as UniformRand>::rand(&mut rng);
            let b = <G2Projective<T> as UniformRand>::rand(&mut rng);
            let s = <Fr<T> as UniformRand>::rand(&mut rng);

            // sa = s * a;
            let mut sa = a;
            sa.mul_assign(s);
            // sb = s * b;
            let mut sb = b;
            sb.mul_assign(s);

            // write sa sb to input
            let mut input = Vec::new();
            sa.into_affine().write(&mut input);
            println!("random g1:{:?}", input.len());
            b.into_affine().write(&mut input);
            println!("random g1:{:?}", input.len());
            // a get negative.
            a.into_affine().neg().write(&mut input);
            println!("random g1:{:?}", input.len());
            sb.into_affine().write(&mut input);
            println!("random g1:{:?}", input.len());

            // e(sa, b) = e(sb, a)
            assert!(T::pairings(&input[..]).expect("pairings failed"));
            println!("test pairings{} success!", i + 1);
        }
    }

    // check pairings
    {
        let g1 = <G1Affine<T> as AffineCurve>::prime_subgroup_generator();
        let g2 = <G2Affine<T> as AffineCurve>::prime_subgroup_generator();

        let a1 = g1;
        let b1 = g2;

        let a2 = g1
            .mul(<Fr<T> as PrimeField>::from_repr(1234u64.into()).unwrap())
            .into_affine();
        let b2 = g2
            .mul(<Fr<T> as PrimeField>::from_repr(1234u64.into()).unwrap())
            .into_affine();

        // -a3
        let a3 = g1.neg();
        let b3 = g2;

        // -a4
        let a4 = g1
            .mul(<Fr<T> as PrimeField>::from_repr(1234u64.into()).unwrap())
            .into_affine()
            .neg();
        let b4 = g2
            .mul(<Fr<T> as PrimeField>::from_repr(1234u64.into()).unwrap())
            .into_affine();

        // a1 * b1  + a2 * b2  + -a1 * b1  + -a2 * b2 = 0
        let expected =
            T::pairing(a1, b1) * &T::pairing(a2, b2) * &T::pairing(a3, b3) * &T::pairing(a4, b4);
        // e(a1*b1) * e(a2*b2) * e(-a1*b1) * e(-a2*b2) = 1
        assert_eq!(<T as PairingEngine>::Fqk::one(), expected);

        // Encode g1s g2s to input.
        let pairings = [(a1, b1), (a2, b2), (a3, b3), (a4, b4)];
        let mut input = Vec::new();
        pairings.iter().for_each(|(g1, g2)| {
            g1.write(&mut input);
            g2.write(&mut input);
        });

        // check pairings operation:(a1*b1) * e(a2*b2) * e(-a1*b1) * e(-a2*b2) == 1 return true
        assert!(T::pairings(&input[..]).unwrap());
        println!("test pairings e(a1*b1)*e(a2*b2)*e(-a1*b1)*e(-a2*b2) success!");
    }
}

/// Test pairing for Curve `T`
pub fn test_pairings<T>()
where
    T: PairingEngine + CurveBasicOperations,
{
    // check pairings
    {
        let g1 = <G1Affine<T> as AffineCurve>::prime_subgroup_generator();
        let g2 = <G2Affine<T> as AffineCurve>::prime_subgroup_generator();

        let a1 = g1;
        let b1 = g2;

        let a2 = g1
            .mul(<Fr<T> as PrimeField>::from_repr(1234u64.into()).unwrap())
            .into_affine();
        let b2 = g2
            .mul(<Fr<T> as PrimeField>::from_repr(4224u64.into()).unwrap())
            .into_affine();

        // -a3
        let a3 = g1.neg();
        let b3 = g2;

        // -a4
        let a4 = g1
            .mul(<Fr<T> as PrimeField>::from_repr(1234u64.into()).unwrap())
            .into_affine()
            .neg();
        let b4 = g2
            .mul(<Fr<T> as PrimeField>::from_repr(4224u64.into()).unwrap())
            .into_affine();

        // a1 * b1  + a2 * b2  + -a1 * b1  + -a2 * b2 = 0
        let expected =
            T::pairing(a1, b1) * &T::pairing(a2, b2) * &T::pairing(a3, b3) * &T::pairing(a4, b4);
        // e(a1*b1) * e(a2*b2) * e(-a1*b1) * e(-a2*b2) = 1
        assert_eq!(<T as PairingEngine>::Fqk::one(), expected);

        // Encode g1s g2s to input.
        let pairings = [(a1, b1), (a2, b2), (a3, b3), (a4, b4)];
        let mut input = Vec::new();
        pairings.iter().for_each(|(g1, g2)| {
            g1.write(&mut input);
            g2.write(&mut input);
        });

        // check pairings operation:(a1*b1) * e(a2*b2) * e(-a1*b1) * e(-a2*b2) == 1 return true
        assert!(T::pairings(&input[..]).unwrap());
        println!("test pairings e(a1*b1)*e(a2*b2)*e(-a1*b1)*e(-a2*b2) success!");
    }
}
