use super::CurveBasicOperations;
use ark_bw6_761::{Fq6, Fr, G1Projective, G2Projective, BW6_761};
use ark_ec::PairingEngine;
use ark_ff::{test_rng, Field, One, PrimeField};
use ark_std::{ops::MulAssign, vec::Vec};
use rand::Rng;
use rustc_hex::FromHex;

impl CurveBasicOperations for BW6_761 {
    const G1_LEN: usize = 193;
    const G2_LEN: usize = 193;
    const SCALAR_LEN: usize = 48;
}

#[test]
fn test_bw6_761_additional() {
    // zero-points additions
    {
        let input:Vec<u8> = FromHex::from_hex(
            "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001").unwrap();

        let res = BW6_761::add(&input[..]).unwrap();

        let expected :Vec<u8> = FromHex::from_hex(
            "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001").unwrap();

        assert_eq!(&expected[..], &res[..]);
        println!("test add1 success!");
    }

    // one-points additions
    {
        // two one-points add encode
        let input1:Vec<u8> = FromHex::from_hex(
            "3db4e566aff388403f60afa6ac285905823e135603dd50677fa20c289a8f75037109eac9a01fd75b909b7247ce547aa146e7c294d2fcdb11ac2055c1fa7f0179c76ff5854bc505eef0271b55b7cfa0e6aebe77a498ce77b2c890a10e025b07016353e9b42d8ffcbaa1d2200bbeb21cad93fbd0ca1981b0b2533205b341f19d9fd4cdc26f0bb93fbe554c7a71315d68cc06b8b57117fab8c5ba0d7eaff1095926a373e5a2d248731ac69e4c888925950f422acc457b63fde674c56f0a4eb85800003db4e566aff388403f60afa6ac285905823e135603dd50677fa20c289a8f75037109eac9a01fd75b909b7247ce547aa146e7c294d2fcdb11ac2055c1fa7f0179c76ff5854bc505eef0271b55b7cfa0e6aebe77a498ce77b2c890a10e025b07016353e9b42d8ffcbaa1d2200bbeb21cad93fbd0ca1981b0b2533205b341f19d9fd4cdc26f0bb93fbe554c7a71315d68cc06b8b57117fab8c5ba0d7eaff1095926a373e5a2d248731ac69e4c888925950f422acc457b63fde674c56f0a4eb8580000").unwrap();

        let res1 = BW6_761::add(&input1[..]).unwrap();

        let expected :Vec<u8> = FromHex::from_hex(
            "1ae80b765e09a7bf87f4b2023abea11d37181e0c78db8e28e88a401acb8808ec34e9dc4dae4501219771ec5f13830e413b3d08f4505a8d71fdceb2fcc539d31d2a136e49dba3cb8e8c66551aea983759853ab88b0d83d07d47574a7c18d3bd002adf2f6db69dc19d17a54df5a6db30710eeb0d97c0b58de174e915018ba823ab03aab81282a66148bf8308dbd821b318c8849baec50d824af6151af31bac63910304c02e7abfadbae8ee5f939e4e35698f6750f0c4ee1703560d5454f513940000").unwrap();
        assert_eq!(res1, expected);
    }
}

#[test]
fn test_bw6_761_scalar_mul() {
    // one-point mul 2 encode
    let input2:Vec<u8> = FromHex::from_hex(
        "3db4e566aff388403f60afa6ac285905823e135603dd50677fa20c289a8f75037109eac9a01fd75b909b7247ce547aa146e7c294d2fcdb11ac2055c1fa7f0179c76ff5854bc505eef0271b55b7cfa0e6aebe77a498ce77b2c890a10e025b07016353e9b42d8ffcbaa1d2200bbeb21cad93fbd0ca1981b0b2533205b341f19d9fd4cdc26f0bb93fbe554c7a71315d68cc06b8b57117fab8c5ba0d7eaff1095926a373e5a2d248731ac69e4c888925950f422acc457b63fde674c56f0a4eb8580000020000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();

    let res2 = BW6_761::scalar_mul(&input2[..]).unwrap();

    let expected :Vec<u8> = FromHex::from_hex(
        "1ae80b765e09a7bf87f4b2023abea11d37181e0c78db8e28e88a401acb8808ec34e9dc4dae4501219771ec5f13830e413b3d08f4505a8d71fdceb2fcc539d31d2a136e49dba3cb8e8c66551aea983759853ab88b0d83d07d47574a7c18d3bd002adf2f6db69dc19d17a54df5a6db30710eeb0d97c0b58de174e915018ba823ab03aab81282a66148bf8308dbd821b318c8849baec50d824af6151af31bac63910304c02e7abfadbae8ee5f939e4e35698f6750f0c4ee1703560d5454f513940000").unwrap();

    assert_eq!(res2, expected);
}

// 30 times pairings
#[test]
fn test_bw6_761_pairing() {
    // test pairings
    for i in 0..5 {
        {
            // hex![(sa,b),(-sb,a)]
            let input:Vec<u8> = FromHex::from_hex(
                "0bd1717c0d2581b3891c9fc55685efd41599eda572fddae589799d66e55c98c2acac0ff19d907f9f2af31cbe540d22d7d2511fb57874d1d30c26e6b65516b74e49acb5a503c4ceafde11e8102d9607eeee181aea7a192fc3e9f99cb40a9ace00261d6e38f43fd71927c736749beb555c1d35f6cc271630d7b02eb061295b38523dc9c7187c4b988540bb7aa01098776e6b0ff698837039227824bedef4c0017ee2f839bad58c3b25aec3aea039ae99f007144a0975d6208787e87718135b0f00008221187269a97f67ddb3b89674821102b3ce296080a856e24e11cc1036ab36fc345bb86ed61128989a5618057fd09127c61a639f4b4419f48c86ab0f2417fb9984abcc92002ced2a5b61c36a5aeea3ad78b1f708052f30da8b6a146538e34c00ff58f077852dd902f9d4038b9f2fe0b5e4d2e70947ade43e8c1a606b945b0c000be2dea0031bb944715227aafd331690abcdf23a772fa039d38b1648ad2da40715026f359edc375ec7f2c1b16fd7aa0296af31b88ff35eb8c499ae11376ac30000f6b690da673fb611abab389029808c0c03ec8db39fc1dc62fb8be6a46a2705cddaf1cd479b594c968c6b6737a66636d4372b65c4b9705dca9f2213eab6a0bf1c9e5ff041e417044df70510c6076193d9465379815171d3d8195971a18555d8005371ce0d880f554598fa9927e6e989a699029cebfd5efd4e448c30ca607e88742a56d482d67cadd19d8c93be0e64ffa2f74ee39e4d389094098e61c9e9b91f132c81f2feae65281711d0ded6eb88848849e0b4611e51ecc4c79a68fa3cb8340000fdabfe45138bd1f8b64406bee66f099aa67de7c1a8708c74760a83706b1ceae9bf6ae4237269971f7ff3403ba64f07819885beafedd3d90fc5bb9e34c2d90a945a5f8f018e78821046fff3c0aa747cbb6e1d6ef92c126d01813d836271e67600df383d4a00a6365566e278fed9dfbd192252c4d14e0b5e21f9d5823033a1de2dd7280d40ffb3f8551ba54e6efa6734a923d9392d1a31fd7e790ddc465b94258c86567ef506a39731c3b2b756f8d349b06495ace6cbf1a4b2c21f396aaee2980000").unwrap();

            // e(sa, b) = e(sb, a)
            // e(sa, b) * e(-sb, a) = 1
            assert!(BW6_761::pairings(&input[..]).expect("pairings failed"));
        }

        // check pairings
        {
            // hex![(a1, b1), (a2, b2), (-a1, b1), (-a2, b2)];
            let pairings_encoded = "3db4e566aff388403f60afa6ac285905823e135603dd50677fa20c289a8f75037109eac9a01fd75b909b7247ce547aa146e7c294d2fcdb11ac2055c1fa7f0179c76ff5854bc505eef0271b55b7cfa0e6aebe77a498ce77b2c890a10e025b07016353e9b42d8ffcbaa1d2200bbeb21cad93fbd0ca1981b0b2533205b341f19d9fd4cdc26f0bb93fbe554c7a71315d68cc06b8b57117fab8c5ba0d7eaff1095926a373e5a2d248731ac69e4c888925950f422acc457b63fde674c56f0a4eb85800001c5f02cd94c130a85b99bfe14fcf1064b054adc2fb6ee900d708d23ccb4869cebab6e100a3173396c7e770ace9cabbc558eb9ff0f1c34e7368a23dab5d1cb4266d0f89131020064c5f11a7c5aa5310d6f960d6692ea852c816b8d94132131001613bc72867a170eb89c6eaf99405ec91a5025a3c3a2daa58c7ff4a50cd6fa93e0023a8ff70c10689b812c7d2db93f264ef3f2933b77ec9949ca5950bc8861d0a16e3ff53278ea7811c18c2ce9acfb726dcd2b6e410eb79819f36617735c31700005e56432f17a05e17b8239bd4e20a277f7b4ea0c17b3ccf821e186f027db66c0c9f0c8d19a47148d4cae52dc01fd4be25824c605a2fe34cc4c99228916463c7436fdca859832ff60f1bfa22d2c397e8b317f8824427e9bed626b36e1be6db0d00ff77df8a3ae314a859e5308a0cecb678701c6f8457388fd2744fc23f77e9e9f2c9ef5c361a3f6784d2d905aad084750420a0a5854b908bb3bb0a6e470b4cb0beab9edee555b352847fb8b63fdd3f62ec331ca4b83f425de5784718e68b64af0000395bd2f69ac70346ef2e6c947f84f05ee91fa5e490e47bcc38cfa049755c336469244435609a7aca6a9eba05d69209945b4ead089273e41b3d19e0bc2bc5c61e60b701b4c3c78e992c63df61812cc813ca7300723d065773cd6cfa0845294800e243b6b05040038a6c3b2b95d8f6fd1f94864ceb50bb51b178fbc77d7ec897632f986e30fc4492f70cb6a6e0e3d8a4c860c741dcc547197376bdeae7eba695cf85453e417d8f52de48e52968abf68f0a531034207a5c3f309533b180bc55f800003db4e566aff388403f60afa6ac285905823e135603dd50677fa20c289a8f75037109eac9a01fd75b909b7247ce547aa146e7c294d2fcdb11ac2055c1fa7f0179c76ff5854bc505eef0271b55b7cfa0e6aebe77a498ce77b2c890a10e025b070128ad164bd270a039e12ddf64aa8b7439a4a81f2095775c63a47662a3802503f959312904d11a9db33ab17fa1bb6b21bafd6afeb3e7c0153e5edb0636479c224a9e4aa2dd1c2d0f38c2a784f9e0f290a9fcd42ebfc4658aea950814f1d62fca00001c5f02cd94c130a85b99bfe14fcf1064b054adc2fb6ee900d708d23ccb4869cebab6e100a3173396c7e770ace9cabbc558eb9ff0f1c34e7368a23dab5d1cb4266d0f89131020064c5f11a7c5aa5310d6f960d6692ea852c816b8d94132131001613bc72867a170eb89c6eaf99405ec91a5025a3c3a2daa58c7ff4a50cd6fa93e0023a8ff70c10689b812c7d2db93f264ef3f2933b77ec9949ca5950bc8861d0a16e3ff53278ea7811c18c2ce9acfb726dcd2b6e410eb79819f36617735c31700005e56432f17a05e17b8239bd4e20a277f7b4ea0c17b3ccf821e186f027db66c0c9f0c8d19a47148d4cae52dc01fd4be25824c605a2fe34cc4c99228916463c7436fdca859832ff60f1bfa22d2c397e8b317f8824427e9bed626b36e1be6db0d008c882075c51c884c291bcfe55b52da6dc787816657c07d438359a5164b2db7a5640f8f3dc29475edbd23f4681c441482e4820ea0b32a43505dde169e2d5acbb1951fa99a99c22fce088e1a428dd8c3cc0ae3564c00872aec91866b159983730000395bd2f69ac70346ef2e6c947f84f05ee91fa5e490e47bcc38cfa049755c336469244435609a7aca6a9eba05d69209945b4ead089273e41b3d19e0bc2bc5c61e60b701b4c3c78e992c63df61812cc813ca7300723d065773cd6cfa0845294800e243b6b05040038a6c3b2b95d8f6fd1f94864ceb50bb51b178fbc77d7ec897632f986e30fc4492f70cb6a6e0e3d8a4c860c741dcc547197376bdeae7eba695cf85453e417d8f52de48e52968abf68f0a531034207a5c3f309533b180bc55f80000";

            let input: Vec<u8> = FromHex::from_hex(pairings_encoded).unwrap();

            // check pairings operation:(a1*b1) * e(a2*b2) * e(-a1*b1) * e(-a2*b2) == 1 return true
            assert!(BW6_761::pairings(&input[..]).unwrap());
        }
    }
}

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

    let ans1 = BW6_761::pairing(sa, b);
    let ans2 = BW6_761::pairing(a, sb);
    let ans3 = BW6_761::pairing(a, b).pow(s.into_repr());

    assert_eq!(ans1, ans2);
    assert_eq!(ans2, ans3);

    assert_ne!(ans1, Fq6::one());
    assert_ne!(ans2, Fq6::one());
    assert_ne!(ans3, Fq6::one());

    assert_eq!(ans1.pow(Fr::characteristic()), Fq6::one());
    assert_eq!(ans2.pow(Fr::characteristic()), Fq6::one());
    assert_eq!(ans3.pow(Fr::characteristic()), Fq6::one());
}
