use crate::{tests::mimc::test_mimc_groth_16, CurveBasicOperations};
use ark_bn254::{Bn254, Fr};
use ark_ff::{Field, FromBytes, ToBytes};
use ark_groth16::{verify_proof, PreparedVerifyingKey, Proof, VerifyingKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::{ops::MulAssign, test_rng, vec::Vec, UniformRand};
use rustc_hex::FromHex;

/// BN254 ADD
pub fn bn254_add() {
    // two one-points add encode
    let input1:Vec<u8> = FromHex::from_hex(
        "01000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000").unwrap();

    let res1 = Bn254::add(&input1[..]).unwrap();

    let expected :Vec<u8> = FromHex::from_hex(
        "d3cf876dc108c2d3a81c8716a91678d9851518685b04859b021a132ee7440603c4a2185a7abf3effc78f53e349a4a6680a9caeb2965f84e7927c0a0e8c73ed1500").unwrap();
    assert_eq!(res1, expected);
}

/// BN254 MUL
pub fn bn254_mul() {
    let input2:Vec<u8> = FromHex::from_hex(
        "0100000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000308f03188746cbabf2f9329f32e6af3c1030c494557ec38d53380f9f5df6d10c").unwrap();

    let res2 = Bn254::mul(&input2[..]).unwrap();

    let expected :Vec<u8> = FromHex::from_hex(
        "1b67a829f225fa2dfd826ed9dfb6c63fbcb52bba72f01dd9dd84e4b426590e2a7a5f8b601e91a6d52374c844ece03b83757aabb9f093d411393febc5e827372200").unwrap();

    assert_eq!(res2, expected);
}

/// BN254 PAIRING
pub fn bn254_pairing() {
    // hex![(sa,b),(-sb,a)]
    let input:Vec<u8> = FromHex::from_hex(
                "cd660ea9af204d56e8536d2fc28c1038e761f1b8e236c0939457b52ab3c3cf11ce10c5d2da7a0cdaf335b66f66107e053c11b5b148b0ea71bac347607fca161400cfc276876571de255ca2cc081c753259f3fac8b28d037e4a17e9505cdcae4322699d38451ba0ef9d69fae88798f6ae96b949ea72a664c835d907bff7d73ced08c211bfd7b8217d9adbcc921ee4a78459e469349f1198db03febd76b79d39741de7671f7c1c8b1cbf4c40415923b9f65822a4001e9d96a8770b791eb74843aa14001b67a829f225fa2dfd826ed9dfb6c63fbcb52bba72f01dd9dd84e4b426590e2acd9df177f8fa79666956a923a5894514e8ddd5c7c5b17ba6f060461b8a262d0e00cfcebd6a33351e4a0d431fc1a7ce2966986a0c993f15ca044a536f7cc73e2518e379338e3f7c8cc191a8d0187244f842338949d93d1ed31a459f76761f5c080a23e18f756349c5075025f1eb76838dacc031c69f46f1298cdbb05f8a436e2d0ee32c918e061ee50b66ca36f12752f96563ad2ad3c4974ca5e1763c4c9dccd10000").unwrap();

    // e(sa, b) = e(sb, a)
    // e(sa, b) * e(-sb, a) = 1
    assert!(Bn254::pairings(&input[..]).expect("pairings failed"));
}

/// BN254 PAIRING SIX
pub fn bn254_pairing_six() {
    bn254_pairing();

    // check pairings
    {
        // hex![(a1, b1), (a2, b2), (-a1, b1), (-a2, b2)];
        let pairings_encoded = "0100000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000edf692d95cbdde46ddda5ef7d422436779445c5e66006a42761e1f12efde0018c212f3aeb785e49712e7a9353349aaf1255dfb31b7bf60723a480d9293938e19aa7dfa6601cce64c7bd3430c69e7d1e38f40cb8d8071ab4aeb6d8cdba55ec8125b9722d1dcdaac55f38eb37033314bbc95330c69ad999eec75f05f58d08906090026052c2b69c54cc8029fcf0f0080f17ff0dad4e843466d144752112ea624960b63cc54e1673e2f2e7a0e0121cb3c7f7a7a2656713fc3fcc68f6f72dae6bdaa1300239480fea1f93725ff9dc1c9f59a62c73ac8dadfe87ca023dd32aa92416a8618d76f11482ffcbcc9a0943306c5cc62390b9ea0c6962420af6ad3e531c9f4802d3faf67b0fe4ee8264a63306f63c3612c1b2a6b26c3f76091167c2e4cbebfa501bd70cfd715766bdf9b512d3f5ef9bb131781b03d0359f7f5c28be067ae88ef2900010000000000000000000000000000000000000000000000000000000000000045fd7cd8168c203c8dca7168916a81975d588181b64550b829a031e1724e643000edf692d95cbdde46ddda5ef7d422436779445c5e66006a42761e1f12efde0018c212f3aeb785e49712e7a9353349aaf1255dfb31b7bf60723a480d9293938e19aa7dfa6601cce64c7bd3430c69e7d1e38f40cb8d8071ab4aeb6d8cdba55ec8125b9722d1dcdaac55f38eb37033314bbc95330c69ad999eec75f05f58d08906090026052c2b69c54cc8029fcf0f0080f17ff0dad4e843466d144752112ea624960be43028f7ae4df10d13bc7047c62d021de3312b10778253f19930bf068c90b91c00239480fea1f93725ff9dc1c9f59a62c73ac8dadfe87ca023dd32aa92416a8618d76f11482ffcbcc9a0943306c5cc62390b9ea0c6962420af6ad3e531c9f4802d3faf67b0fe4ee8264a63306f63c3612c1b2a6b26c3f76091167c2e4cbebfa501bd70cfd715766bdf9b512d3f5ef9bb131781b03d0359f7f5c28be067ae88ef2900";

        let input: Vec<u8> = FromHex::from_hex(pairings_encoded).unwrap();

        // check pairings operation:(a1*b1) * e(a2*b2) * e(-a1*b1) * e(-a2*b2) == 1 return true
        assert!(Bn254::pairings(&input[..]).unwrap());
    }
}

pub fn bn254_verify() {
    let proof = "cf3bb657065cd0878eabc53790c5fa3680cad24db586bab799e3ae790217c61b0227ad22191396e6ffe7aff14de91d7801e943f836de399b5fd5c8312086440c036648b695ffa27790f2f2a63fcf4c0b130e6b88ac786e3351fc6f13452d18837a922013b86567d6d48ff65baf8cdd0558c52db98f27b6c216df96f8aa1d4d2d";
    let vk = "cc165291944a7b617bad2e6f1560f89f4e7eb42f154333b730e2f2da7828919d94b31d9ebcca3957fff3d94960b0eba2197fabaaa68776e767e8785f63b4541d456d755240d72dae2ee0452c0cd705d7312b9a1d17cff916e87c763d459a5504f00c5d68983d7c029a66ae498a1cc6a2423bf67760bbf922ea7a937a87705e158db79bbff4eb9516d5342ce8cd8d09f68fa094434edceebbb43383fc415d9b23f70bfb6ec84456af219bcd1d682133db3db80b0ae54cf31879e74a0ae547ec0ef92f56757f4a7976c50dc3d538cb1b4ed1736d4e8e8734d5cbf85b2ac8a93d0b020000000000000003c2e806d904d8789f0caf1276fc4897bbf2bbb2e7243df2e4bf941ce61679844d609323ed564943ba7dfd3b362cc85abf7691dd9957c613fc0d282f258f4d17";
    let image = "829d3d572251103bac02b39e397c657c0b1f372bfc4d1b74f6c23de4993b4824";

    let proof_input: Vec<u8> = FromHex::from_hex(proof).unwrap();
    let vk_input: Vec<u8> = FromHex::from_hex(vk).unwrap();
    let image_input: Vec<u8> = FromHex::from_hex(image).unwrap();

    let proof = Proof::<Bn254>::deserialize(&*proof_input).expect("Proof deserialize fail:");
    let vk = VerifyingKey::<Bn254>::deserialize(&*vk_input).expect("vk deserialize fail");
    let pvk = PreparedVerifyingKey::<Bn254>::from(vk);
    let image = Fr::read(&*image_input).unwrap();

    assert!(verify_proof(&pvk, &proof, &[image]).unwrap());
}

#[test]
pub fn test_bn254_verify() {
    bn254_verify();
}

#[test]
pub fn test_bn254_groth16() {
    test_mimc_groth_16::<Bn254>();
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
    }

    // one-points additions
    bn254_add();
}

#[test]
fn test_bn256_scalar_mul() {
    bn254_mul();
}

// 30 times pairings
#[test]
fn test_bn256_pairing() {
    // test pairings
    for i in 0..5 {
        bn254_pairing_six();
    }
}
