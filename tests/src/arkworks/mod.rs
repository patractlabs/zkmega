mod bench;
mod ops;
mod verify;

use ark_ec::{AffineCurve, PairingEngine, ProjectiveCurve};
use ark_ff::{FromBytes, One, PrimeField, ToBytes, Zero};
use ark_serialize::SerializationError;
use ark_std::{
    io::{Error, ErrorKind},
    ops::{MulAssign, Neg},
    str::FromStr,
    test_rng,
    vec::Vec,
    UniformRand,
};
use arkworks::CurveBasicOperations;
use rand::Rng;

type G1Affine<T> = <T as PairingEngine>::G1Affine;
type G2Affine<T> = <T as PairingEngine>::G2Affine;
type G1Projective<T> = <T as PairingEngine>::G1Projective;
type G2Projective<T> = <T as PairingEngine>::G2Projective;
type Fr<T> = <T as PairingEngine>::Fr;

fn all_curve_three_operations_test<T>()
where
    T: CurveBasicOperations + PairingEngine,
{
    // zero-points additions
    {
        let mut input = Vec::new();
        <G1Affine<T> as Zero>::zero().write(&mut input);
        <G1Affine<T> as Zero>::zero().write(&mut input);

        println!("1 zero: {}", hex::encode(&input));
        let mut expected = Vec::new();
        <G1Affine<T> as Zero>::zero().write(&mut expected);
        println!("1 expected: {}", hex::encode(&expected));

        let res = T::add(&input[..]).unwrap();
        assert_eq!(&expected[..], &res[..]);
        println!("test add1 success!");
    }

    // one-points additions
    {
        let mut input1 = Vec::new();
        <G1Affine<T> as AffineCurve>::prime_subgroup_generator().write(&mut input1);
        <G1Affine<T> as AffineCurve>::prime_subgroup_generator().write(&mut input1);
        println!("2 input1: {}", hex::encode(&input1));

        let mut input2 = Vec::new();
        <G1Affine<T> as AffineCurve>::prime_subgroup_generator().write(&mut input2);
        <Fr<T> as PrimeField>::from_repr(2u64.into()).map(|x| x.write(&mut input2));
        println!("2 input2: {}", hex::encode(&input2));

        let res1 = T::add(&input1[..]).unwrap();
        let res2 = T::mul(&input2[..]).unwrap();
        println!("2 res1: {}", hex::encode(&res1));
        println!("2 res2: {}", hex::encode(&res2));

        assert_eq!(res1, res2);
        println!("test add2 success!");
    }

    // Prime subgroup generator additions check prime subgroup generator * 2(scalar_mul)
    {
        let mut input1 = Vec::new();
        <G1Affine<T> as AffineCurve>::prime_subgroup_generator().write(&mut input1);
        println!("3 input1: {}", hex::encode(&input1));

        let mut input2 = Vec::new();
        <G1Affine<T> as AffineCurve>::prime_subgroup_generator().write(&mut input2);
        <Fr<T> as FromStr>::from_str("2").map(|x| x.write(&mut input2));
        println!("3 input2: {}", hex::encode(&input2));

        let res1 = T::add(&input1.repeat(2)[..]).expect("Generator add failed");
        let res2 = T::mul(&input2[..]).expect("Generator scalar_mul 2 failed");

        let mut res3 = Vec::new();
        <G1Affine<T> as AffineCurve>::prime_subgroup_generator()
            .into_projective()
            .double()
            .into_affine()
            .write(&mut res3);

        println!("3 res1: {}", hex::encode(&res1));
        println!("3 res2: {}", hex::encode(&res2));
        println!("3 res3: {}", hex::encode(&res3));
        // prime_subgroup_generator + prime_subgroup_generator = prime_subgroup_generator * 2
        assert_eq!(res1, res3);
        println!("test add3 success!");
        assert_eq!(res2, res3);
        println!("test scalar_mul1 success!");
    }

    // test pairings
    {
        for i in 0..1 {
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
            println!("4 input: {}", hex::encode(&input));

            // e(sa, b) = e(sb, a)
            assert!(<T as CurveBasicOperations>::pairings(&input[..]).expect("pairings failed"));
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
        let expected = <T as PairingEngine>::pairing(a1, b1)
            * <T as PairingEngine>::pairing(a2, b2)
            * <T as PairingEngine>::pairing(a3, b3)
            * <T as PairingEngine>::pairing(a4, b4);
        // e(a1*b1) * e(a2*b2) * e(-a1*b1) * e(-a2*b2) = 1
        assert_eq!(<T as PairingEngine>::Fqk::one(), expected);

        // Encode g1s g2s to input.
        let pairings = [(a1, b1), (a2, b2), (a3, b3), (a4, b4)];
        let mut input = Vec::new();
        pairings.iter().for_each(|(g1, g2)| {
            g1.write(&mut input);
            g2.write(&mut input);
        });
        println!("5 input: {}", hex::encode(&input));

        // check pairings operation:(a1*b1) * e(a2*b2) * e(-a1*b1) * e(-a2*b2) == 1 return true
        assert!(<T as CurveBasicOperations>::pairings(&input[..]).unwrap());
        println!("test pairings e(a1*b1)*e(a2*b2)*e(-a1*b1)*e(-a2*b2) success!");
    }
}
