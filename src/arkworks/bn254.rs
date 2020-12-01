use super::CurveBasicOperations;
use ark_bn254::{Bn254, Fq, Fq12, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{AffineCurve, PairingEngine, ProjectiveCurve};
use ark_ff::{test_rng, Field, One, PrimeField, ToBytes, Zero};
use ark_std::ops::{MulAssign, Neg};
use ark_std::str::FromStr;
use rand::Rng;

impl CurveBasicOperations for Bn254 {
    const FQ_LEN: usize = 32;
    const SCALAR_LEN: usize = 32;
}

#[test]
fn test_all_bn254() {
    // zero-points additions
    {
        let mut input = Vec::new();
        G1Affine::zero().write(&mut input);
        G1Affine::zero().write(&mut input);

        let mut expected = Vec::new();
        G1Affine::zero().write(&mut expected);

        let res = Bn254::add(&input[..]).unwrap();
        assert_eq!(&expected[..], &res[..]);
        println!("test add1 success!");
    }

    // one-points additions
    {
        let mut input1 = Vec::new();
        G1Affine::prime_subgroup_generator().write(&mut input1);
        G1Affine::prime_subgroup_generator().write(&mut input1);

        let mut input2 = Vec::new();

        G1Affine::prime_subgroup_generator().write(&mut input2);
        Fr::from_str("2").unwrap().write(&mut input2);

        let res1 = Bn254::add(&input1[..]).unwrap();
        let res2 = Bn254::scalar_mul(&input2[..]).unwrap();
        assert_eq!(res1, res2);
        println!("test add2 success!");
    }

    // Prime subgroup generator additions check prime subgroup generator * 2(scalar_mul)
    {
        let mut input1 = [0u8; 65];
        G1Affine::prime_subgroup_generator().write(&mut input1[..]);

        let mut input2 = Vec::new();
        G1Affine::prime_subgroup_generator().write(&mut input2);
        Fr::from_str("2").unwrap().write(&mut input2);

        let res1 = Bn254::add(&input1.repeat(2)[..]).expect("Generator add failed");
        let res2 = Bn254::scalar_mul(&input2[..]).expect("Generator scalar_mul 2 failed");

        let mut res3 = [0u8; 65];
        G1Affine::prime_subgroup_generator()
            .into_projective()
            .double()
            .into_affine()
            .write(&mut res3[..]);

        // prime_subgroup_generator + prime_subgroup_generator = prime_subgroup_generator * 2
        assert_eq!(res1, res3);
        println!("test add3 success!");
        assert_eq!(res2, res3);
        println!("test scalar_mul1 success!");
    }

    // test pairings
    {
        for _ in 0..100 {
            let mut rng = test_rng();
            let a: G1Projective = rng.gen();
            let b: G2Projective = rng.gen();
            let s: Fr = rng.gen();

            // sa = s * a;
            let mut sa = a;
            sa.mul_assign(s);
            // sb = s * b;
            let mut sb = b;
            sb.mul_assign(s);

            // write sa sb to input
            let mut input = [0u8; 194 * 2];
            sa.into_affine().write(&mut input[0..65]);
            b.into_affine().write(&mut input[65..194]);
            // a get negative.
            a.into_affine().neg().write(&mut input[194..194 + 65]);
            sb.into_affine().write(&mut input[194 + 65..]);

            // e(sa, b) = e(sb, a)
            assert!(Bn254::pairings(&input[..]).expect("pairings failed"));
            println!("test pairings2 success!");
        }
    }

    // check pairings
    {
        let a1 = G1Affine::prime_subgroup_generator();
        let b1 = G2Affine::prime_subgroup_generator();

        let a2 = G1Affine::prime_subgroup_generator()
            .mul(Fr::from_str("1234").unwrap())
            .into_affine();
        let b2 = G2Affine::prime_subgroup_generator()
            .mul(Fr::from_str("4224").unwrap())
            .into_affine();

        // -a3
        let a3 = G1Affine::prime_subgroup_generator().neg();
        let b3 = G2Affine::prime_subgroup_generator();

        // -a4rand
        let a4 = G1Affine::prime_subgroup_generator()
            .mul(Fr::from_str("1234").unwrap())
            .into_affine()
            .neg();
        let b4 = G2Affine::prime_subgroup_generator()
            .mul(Fr::from_str("4224").unwrap())
            .into_affine();

        // a1 * b1  + a2 * b2  + -a1 * b1  + -a2 * b2 = 0
        let expected = Bn254::pairing(a1, b1)
            * Bn254::pairing(a2, b2)
            * Bn254::pairing(a3, b3)
            * Bn254::pairing(a4, b4);

        let pairings = [(a1, b1), (a2, b2), (a3, b3), (a4, b4)];

        let mut input = [0u8; 194 * 4];
        pairings.iter().enumerate().for_each(|(i, (a, b))| {
            a.write(&mut input[194 * i..194 * i + 65]);
            b.write(&mut input[194 * i + 65..194 * (i + 1)]);
        });

        // e(a1*b1) + e(a2*b2) + e(-a1*b1) + e(-a2*b2) = 1
        assert_eq!(Fq12::one(), expected);
        // check e(a1*b1) + e(a2*b2) + e(-a1*b1) + e(-a2*b2) == 1 return true
        assert!(Bn254::pairings(&input[..]).unwrap());
        println!("test pairings1 success!");
    }
}
