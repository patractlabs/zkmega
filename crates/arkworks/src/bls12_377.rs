use super::{all_curve_three_operations_test, CurveBasicOperations};
use ark_bls12_377::{Bls12_377, Fq12, Fr, G1Projective, G2Projective};
use ark_ec::PairingEngine;
use ark_ff::{
    fields::{Field, PrimeField},
    test_rng, One,
};
use ark_std::ops::MulAssign;
use rand::Rng;
use rustc_hex::FromHex;

impl CurveBasicOperations for Bls12_377 {
    const G1_LEN: usize = 97;
    const G2_LEN: usize = 193;
    const SCALAR_LEN: usize = 32;
}

#[test]
fn test_bls12_377() {
    all_curve_three_operations_test::<Bls12_377>();
    // test_pairings::<Bls12_377>();
}

#[test]
fn test_bls12_377_additional() {
    // zero-points additions
    {
        let input:Vec<u8> = FromHex::from_hex(
            "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001").unwrap();

        let res = Bls12_377::add(&input[..]).unwrap();

        let expected :Vec<u8> = FromHex::from_hex(
            "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001").unwrap();

        assert_eq!(&expected[..], &res[..]);
        println!("test add1 success!");
    }

    // one-points additions
    {
        // two one-points add encode
        let input1:Vec<u8> = FromHex::from_hex(
            "efe91bb26eb1b9ea4e39cdff121548d55ccb37bdc8828218bb419daa2c1e958554ff87bf2562fcc8670a74fede488800a68e9c5555de82fd1a59a934363dfec20523b84fd42a186dd9523eca48b37fbdc4eeaf305d4f671fff2e10c5694a910100efe91bb26eb1b9ea4e39cdff121548d55ccb37bdc8828218bb419daa2c1e958554ff87bf2562fcc8670a74fede488800a68e9c5555de82fd1a59a934363dfec20523b84fd42a186dd9523eca48b37fbdc4eeaf305d4f671fff2e10c5694a910100").unwrap();

        let res1 = Bls12_377::add(&input1[..]).unwrap();

        let expected :Vec<u8> = FromHex::from_hex(
            "9063416a6ded7a8590dc816765610688551930a2c9970ee97e4b2addf3f7617eed52544b5adb6e05919e93413145ed00edc7d727875edde2a75ced75563fa2d67944c635f1120be8ca61c542aecd99ad37131713186004aee5c87b71b9b0cf0000").unwrap();
        assert_eq!(res1, expected);
        println!("test add2 success!");
    }
}

#[test]
fn test_bls12_377_scalar_mul() {
    // one-point mul 2 encode
    let input2:Vec<u8> = FromHex::from_hex(
        "efe91bb26eb1b9ea4e39cdff121548d55ccb37bdc8828218bb419daa2c1e958554ff87bf2562fcc8670a74fede488800a68e9c5555de82fd1a59a934363dfec20523b84fd42a186dd9523eca48b37fbdc4eeaf305d4f671fff2e10c5694a9101000200000000000000000000000000000000000000000000000000000000000000").unwrap();

    let res2 = Bls12_377::scalar_mul(&input2[..]).unwrap();

    let expected :Vec<u8> = FromHex::from_hex(
        "9063416a6ded7a8590dc816765610688551930a2c9970ee97e4b2addf3f7617eed52544b5adb6e05919e93413145ed00edc7d727875edde2a75ced75563fa2d67944c635f1120be8ca61c542aecd99ad37131713186004aee5c87b71b9b0cf0000").unwrap();

    assert_eq!(res2, expected);
    println!("test add2 success!");
}

// 30 times pairings
#[test]
fn test_bls12_377_pairing() {
    for i in 0..5 {
        // test pairings
        {
            // vec![sa,b,-sb,a]
            let input: Vec<u8> = FromHex::from_hex(
                "3f91af9f54c6c46762fa432ba77fdc7bc170b508b66af45b42bb39b1c77cd513111a2930586b2cb1fd86fdd2034ce800512eb2b6d18c0a2d0e88beca80edce8ffd9e216c15a677a313823bfe97efd8e7cbcde852ceeb77fc78fc6242e8292c010034f4f5f69b22533deac9d117c88d719c189f9a1a82279ba185252018196b74e540414fb37a699046fc643b496411cf002f91afe9bd9b9b0ab98fcf4b6aea16b43d645050c72aac47dfab448adf1a18f37e2c93d68dae425bb9630a796aa07f0143c24ddff137582cfe320e4dcb0e5639bb3eb7ab164bc269e2c14743f48d9a153035353da6abbcbde93ea4eacb994d01c1bf8d711b6ffcc677b0509da0f47de1c131e78022efa63dd03d38b54c490925bbf4bcc063ddac4171057ee4a78b5d000033c670abc1eddeae7b766b48a816c0d6dc6c877df39ffc10f76458821d553f4390997c2d9bcca0f8444f7db19e576f00f3dde8521b4a3163fd91cdaaf7944f8a97e252a550a1085f38e866c2e01b123946eb6388e3d121ac51c29204ec8f480000b5beea6c8d3deecf87c7d73d3a6a2fa78f3472537b26cde5dae5e2b293b4319905ce2618199b265db72cdf83da2e30002eac94d62b172dfd518958607ff1bada47548a37682a906d8e7b474ebf251b1b2ff343e9830bc3a7c6620c34a1114500fa65f92d3cf847fd4d23820a53234fbe1d89584777ae99ed52cc15541bcbcc916191343f14c0a01673411e16fc2b19001040d139da62b80204f0906530b36c072462f6dbfc5a362d8fbcf425c6be60f36c958d6d0894bffec5639b6e20e9ab0000").unwrap();

            // e(sa, b) = e(sb, a)
            // e(sa, b) * e(-sb, a) = 1
            assert!(Bls12_377::pairings(&input[..]).expect("pairings failed"));
            println!("test pairings{} success!", i + 1);
        }

        // check pairings
        {
            // hex![(a1, b1), (a2, b2), (-a1, b1), (-a2, b2)];
            let pairings_encoded = "efe91bb26eb1b9ea4e39cdff121548d55ccb37bdc8828218bb419daa2c1e958554ff87bf2562fcc8670a74fede488800a68e9c5555de82fd1a59a934363dfec20523b84fd42a186dd9523eca48b37fbdc4eeaf305d4f671fff2e10c5694a9101009651007c8fe4e374025453bb529f88719b6bdb57f501a57e31503e2071f065c5011d84a3a23096c8fe85c771be808401fe6aa16efafe6bb2e66ff7bf8499f85cdec99907ce3e22e7cbce5166ee772753d540b1b1515adc70314000e74060ea00df4dfd09440994f02e7c8c6d8888cff204d232f882c258e4589ab47472ed03deb4efb2cb6b7360d97b6f445d660d6900938feb85d1cda1d90b27525e3fb87942c204e3ce1ab06324f11b593dac11ef61aa701a15a39d549e185583d29f16f80000aafaf0e79aa79ea5f16b5a17763d6c5e2b46fadc470b91c24f3868b9fe80cfeb4611cc935337b357fed7e02ae0073c0137eacc8c8f56bc812783c1ce053eef88382315bd84029d958d3ced6e237b1979e7f66aed4f20292f0948e43e4ade7d00007c1da03901590e708ee46188831e096b478d9aa924bb37827d7ad6711c43ec10c4e93c226235d73358cb351b30214f0137d32b7b86eb47d816778557ae504b58d998457211f9694bd2646a744fb43718d9db8de18fed5eff1edb59bb14c6c6002f0c8568aad2ba7431617bf2e84fb57e05d7c2d42206706e1300062d4c0a49206865bb6d08e37cc7bcc258580bd55f0081584fbeb74ae1e3e1afa480c070f035675216f6fc38fccc9f073a93e63024aaed71a90761ae49054128a583c7c7090000efe91bb26eb1b9ea4e39cdff121548d55ccb37bdc8828218bb419daa2c1e958554ff87bf2562fcc8670a74fede4888005b7163aaaae18587e5a656fb0d200d54fa24516a5b37dbb1b5c0b636aa26a35c765af13b63b6d3a6ebe1b452dcef1c00009651007c8fe4e374025453bb529f88719b6bdb57f501a57e31503e2071f065c5011d84a3a23096c8fe85c771be808401fe6aa16efafe6bb2e66ff7bf8499f85cdec99907ce3e22e7cbce5166ee772753d540b1b1515adc70314000e74060ea00df4dfd09440994f02e7c8c6d8888cff204d232f882c258e4589ab47472ed03deb4efb2cb6b7360d97b6f445d660d6900938feb85d1cda1d90b27525e3fb87942c204e3ce1ab06324f11b593dac11ef61aa701a15a39d549e185583d29f16f80000aafaf0e79aa79ea5f16b5a17763d6c5e2b46fadc470b91c24f3868b9fe80cfeb4611cc935337b357fed7e02ae0073c01ca15337370694c03d97c3e613e1f1c8ec724f4fcaa5f568901d70792cf5e09a15352367f70e51197e1c8e0d8fb5b3001007c1da03901590e708ee46188831e096b478d9aa924bb37827d7ad6711c43ec10c4e93c226235d73358cb351b30214f0137d32b7b86eb47d816778557ae504b58d998457211f9694bd2646a744fb43718d9db8de18fed5eff1edb59bb14c6c6002f0c8568aad2ba7431617bf2e84fb57e05d7c2d42206706e1300062d4c0a49206865bb6d08e37cc7bcc258580bd55f0081584fbeb74ae1e3e1afa480c070f035675216f6fc38fccc9f073a93e63024aaed71a90761ae49054128a583c7c7090000";

            let input: Vec<u8> = FromHex::from_hex(pairings_encoded).unwrap();

            // check pairings operation:(a1*b1) * e(a2*b2) * e(-a1*b1) * e(-a2*b2) == 1 return true
            assert!(Bls12_377::pairings(&input[..]).unwrap());
            println!("test pairings e(a1*b1)*e(a2*b2)*e(-a1*b1)*e(-a2*b2) success!");
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

    let ans1 = Bls12_377::pairing(sa, b);
    let ans2 = Bls12_377::pairing(a, sb);
    let ans3 = Bls12_377::pairing(a, b).pow(s.into_repr());

    assert_eq!(ans1, ans2);
    assert_eq!(ans2, ans3);

    assert_ne!(ans1, Fq12::one());
    assert_ne!(ans2, Fq12::one());
    assert_ne!(ans3, Fq12::one());

    assert_eq!(ans1.pow(Fr::characteristic()), Fq12::one());
    assert_eq!(ans2.pow(Fr::characteristic()), Fq12::one());
    assert_eq!(ans3.pow(Fr::characteristic()), Fq12::one());
}
