use super::{all_curve_three_operations_test, CurveBasicOperations};
use ark_bn254::{Bn254, Fq12, Fr, G1Projective, G2Projective};
use ark_ec::PairingEngine;
use ark_ff::{test_rng, Field, One, PrimeField};
use ark_std::ops::MulAssign;
use rand::Rng;
use rustc_hex::FromHex;

impl CurveBasicOperations for Bn254 {
    const G1_LEN: usize = 65;
    const G2_LEN: usize = 129;
    const SCALAR_LEN: usize = 32;
}

#[test]
fn test_bn256() {
    all_curve_three_operations_test::<Bn254>();
    // test_pairings::<Bn254>();
}

#[test]
fn test_bn256_additional() {
    // zero-points additions
    {
        let input:Vec<u8> = FromHex::from_hex(
            "00000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001").unwrap();

        let res = Bn254::add(&input[..]).unwrap();

        let expected :Vec<u8> = FromHex::from_hex(
            "0000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001").unwrap();

        assert_eq!(&expected[..], &res[..]);
        println!("test add1 success!");
    }

    // one-points additions
    {
        // two one-points add encode
        let input1:Vec<u8> = FromHex::from_hex(
            "01000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000").unwrap();

        let res1 = Bn254::add(&input1[..]).unwrap();

        let expected :Vec<u8> = FromHex::from_hex(
            "d3cf876dc108c2d3a81c8716a91678d9851518685b04859b021a132ee7440603c4a2185a7abf3effc78f53e349a4a6680a9caeb2965f84e7927c0a0e8c73ed1500").unwrap();
        assert_eq!(res1, expected);
        println!("test add2 success!");
    }
}

#[test]
fn test_bn256_scalar_mul() {
    // one-point mul 2 encode
    let input2:Vec<u8> = FromHex::from_hex(
        "01000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000").unwrap();

    let res2 = Bn254::scalar_mul(&input2[..]).unwrap();

    let expected :Vec<u8> = FromHex::from_hex(
        "d3cf876dc108c2d3a81c8716a91678d9851518685b04859b021a132ee7440603c4a2185a7abf3effc78f53e349a4a6680a9caeb2965f84e7927c0a0e8c73ed1500").unwrap();

    assert_eq!(res2, expected);
    println!("test add2 success!");
}

// 30 times pairings
#[test]
fn test_bn256_pairing() {
    // test pairings
    for i in 0..5 {
        {
            // hex![(sa,b),(-sb,a)]
            let input:Vec<u8> = FromHex::from_hex(
                "cd660ea9af204d56e8536d2fc28c1038e761f1b8e236c0939457b52ab3c3cf11ce10c5d2da7a0cdaf335b66f66107e053c11b5b148b0ea71bac347607fca161400cfc276876571de255ca2cc081c753259f3fac8b28d037e4a17e9505cdcae4322699d38451ba0ef9d69fae88798f6ae96b949ea72a664c835d907bff7d73ced08c211bfd7b8217d9adbcc921ee4a78459e469349f1198db03febd76b79d39741de7671f7c1c8b1cbf4c40415923b9f65822a4001e9d96a8770b791eb74843aa14001b67a829f225fa2dfd826ed9dfb6c63fbcb52bba72f01dd9dd84e4b426590e2acd9df177f8fa79666956a923a5894514e8ddd5c7c5b17ba6f060461b8a262d0e00cfcebd6a33351e4a0d431fc1a7ce2966986a0c993f15ca044a536f7cc73e2518e379338e3f7c8cc191a8d0187244f842338949d93d1ed31a459f76761f5c080a23e18f756349c5075025f1eb76838dacc031c69f46f1298cdbb05f8a436e2d0ee32c918e061ee50b66ca36f12752f96563ad2ad3c4974ca5e1763c4c9dccd10000").unwrap();

            // e(sa, b) = e(sb, a)
            // e(sa, b) * e(-sb, a) = 1
            assert!(Bn254::pairings(&input[..]).expect("pairings failed"));
            println!("test pairings{} success!", i + 1);
        }

        // check pairings
        {
            // hex![(a1, b1), (a2, b2), (-a1, b1), (-a2, b2)];
            let pairings_encoded = "0100000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000edf692d95cbdde46ddda5ef7d422436779445c5e66006a42761e1f12efde0018c212f3aeb785e49712e7a9353349aaf1255dfb31b7bf60723a480d9293938e19aa7dfa6601cce64c7bd3430c69e7d1e38f40cb8d8071ab4aeb6d8cdba55ec8125b9722d1dcdaac55f38eb37033314bbc95330c69ad999eec75f05f58d08906090026052c2b69c54cc8029fcf0f0080f17ff0dad4e843466d144752112ea624960b63cc54e1673e2f2e7a0e0121cb3c7f7a7a2656713fc3fcc68f6f72dae6bdaa1300239480fea1f93725ff9dc1c9f59a62c73ac8dadfe87ca023dd32aa92416a8618d76f11482ffcbcc9a0943306c5cc62390b9ea0c6962420af6ad3e531c9f4802d3faf67b0fe4ee8264a63306f63c3612c1b2a6b26c3f76091167c2e4cbebfa501bd70cfd715766bdf9b512d3f5ef9bb131781b03d0359f7f5c28be067ae88ef2900010000000000000000000000000000000000000000000000000000000000000045fd7cd8168c203c8dca7168916a81975d588181b64550b829a031e1724e643000edf692d95cbdde46ddda5ef7d422436779445c5e66006a42761e1f12efde0018c212f3aeb785e49712e7a9353349aaf1255dfb31b7bf60723a480d9293938e19aa7dfa6601cce64c7bd3430c69e7d1e38f40cb8d8071ab4aeb6d8cdba55ec8125b9722d1dcdaac55f38eb37033314bbc95330c69ad999eec75f05f58d08906090026052c2b69c54cc8029fcf0f0080f17ff0dad4e843466d144752112ea624960be43028f7ae4df10d13bc7047c62d021de3312b10778253f19930bf068c90b91c00239480fea1f93725ff9dc1c9f59a62c73ac8dadfe87ca023dd32aa92416a8618d76f11482ffcbcc9a0943306c5cc62390b9ea0c6962420af6ad3e531c9f4802d3faf67b0fe4ee8264a63306f63c3612c1b2a6b26c3f76091167c2e4cbebfa501bd70cfd715766bdf9b512d3f5ef9bb131781b03d0359f7f5c28be067ae88ef2900";

            let input: Vec<u8> = FromHex::from_hex(pairings_encoded).unwrap();

            // check pairings operation:(a1*b1) * e(a2*b2) * e(-a1*b1) * e(-a2*b2) == 1 return true
            assert!(Bn254::pairings(&input[..]).unwrap());
            println!("test pairings e(a1*b1)*e(a2*b2)*e(-a1*b1)*e(-a2*b2) success!");
        }
    }
}

//
#[test]
fn test_wasm_pairing() {
    let mut rng = test_rng();
    let a: G1Projective = rng.gen();
    let b: G2Projective = rng.gen();
    let s: Fr = rng.gen();

    let mut sa = a;
    sa.mul_assign(s);
    let mut sb = b;
    sb.mul_assign(s);

    let ans1 = Bn254::pairing(sa, b);
    let ans2 = Bn254::pairing(a, sb);
    let ans3 = Bn254::pairing(a, b).pow(s.into_repr());

    assert_eq!(ans1, ans2);
    assert_eq!(ans2, ans3);

    assert_ne!(ans1, Fq12::one());
    assert_ne!(ans2, Fq12::one());
    assert_ne!(ans3, Fq12::one());

    assert_eq!(ans1.pow(Fr::characteristic()), Fq12::one());
    assert_eq!(ans2.pow(Fr::characteristic()), Fq12::one());
    assert_eq!(ans3.pow(Fr::characteristic()), Fq12::one());
}
