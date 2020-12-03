use super::{all_curve_three_operations_test, CurveBasicOperations};
use ark_bls12_381::{Bls12_381, Fq12, Fr, G1Projective, G2Projective};
use ark_ec::PairingEngine;
use ark_ff::{test_rng, Field, One, PrimeField};
use ark_std::ops::MulAssign;
use rand::Rng;
use rustc_hex::FromHex;

impl CurveBasicOperations for Bls12_381 {
    const G1_LEN: usize = 97;
    const G2_LEN: usize = 193;
    const SCALAR_LEN: usize = 32;
}

#[test]
fn test_bls12_381() {
    all_curve_three_operations_test::<Bls12_381>();
    // test_pairings::<Bls12_381>();
}

#[test]
fn test_bls12_381_additional() {
    // zero-points additions
    {
        let input:Vec<u8> = FromHex::from_hex(
            "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001\
                    000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001\
                    00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001\
                    000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001").unwrap();

        let res = Bls12_381::add(&input[..]).unwrap();

        let expected :Vec<u8> = FromHex::from_hex(
            "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001\
                    000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001").unwrap();

        assert_eq!(&expected[..], &res[..]);
        println!("test add1 success!");
    }

    // one-points additions
    {
        // two one-points add encode
        let input1:Vec<u8> = FromHex::from_hex(
            "bbc622db0af03afbef1a7af93fe8556c58ac1b173f3a4ea105b974974f8c68c30faca94f8c63952694d79731a7d3f117e1\
                    e7c5462923aa0ce48a88a244c73cd0edb3042ccb18db00f60ad0d595e0f5fce48a1d74ed309ea0f1a0aae381f4b30800\
                   bbc622db0af03afbef1a7af93fe8556c58ac1b173f3a4ea105b974974f8c68c30faca94f8c63952694d79731a7d3f117e1\
                    e7c5462923aa0ce48a88a244c73cd0edb3042ccb18db00f60ad0d595e0f5fce48a1d74ed309ea0f1a0aae381f4b30800").unwrap();

        let res1 = Bls12_381::add(&input1[..]).unwrap();

        let expected :Vec<u8> = FromHex::from_hex(
            "4e0fbf29558c9ac3427c1c8fbb758fe22aa658c30a2d90432501289130db21970c45a950ebc8088846674d90eacb720528\
                    9d7479198886ba1bbd16cdd4d9564c6ad75f1d02b93bf761e47086cb3eba22388e9d7773a6fd22a373c6ab8c9d6a1600").unwrap();
        assert_eq!(res1, expected);
        println!("test add2 success!");
    }
}

#[test]
fn test_bls12_381_scalar_mul() {
    // one-point mul 2 encode
    let input2:Vec<u8> = FromHex::from_hex(
        "bbc622db0af03afbef1a7af93fe8556c58ac1b173f3a4ea105b974974f8c68c30faca94f8c63952694d79731a7d3f117e1\
                e7c5462923aa0ce48a88a244c73cd0edb3042ccb18db00f60ad0d595e0f5fce48a1d74ed309ea0f1a0aae381f4b308000\
                200000000000000000000000000000000000000000000000000000000000000").unwrap();

    let res2 = Bls12_381::scalar_mul(&input2[..]).unwrap();

    let expected :Vec<u8> = FromHex::from_hex(
        "4e0fbf29558c9ac3427c1c8fbb758fe22aa658c30a2d90432501289130db21970c45a950ebc8088846674d90eacb720528\
                9d7479198886ba1bbd16cdd4d9564c6ad75f1d02b93bf761e47086cb3eba22388e9d7773a6fd22a373c6ab8c9d6a1600").unwrap();

    assert_eq!(res2, expected);
    println!("test add2 success!");
}

// 30 times pairings
#[test]
fn test_bls12_381_pairing() {
    for i in 0..5 {
        // test pairings
        {
            // vec![sa,b,-sb,a]
            let input: Vec<u8> = FromHex::from_hex(
                "0b198686e7b0d46c9857744a328590a0a4368724b79e3c747df05f90ef692a36dbef2f5643a3789c29f536d58068cb162e\
                       586c47ee9fc22748c5b6ca5e125dcf758bb3899a581e58c0fa5bc8c6be87edeb1bd21125524eb45c750da4a9d278030009\
                       c3078d5c5886fb948bbda03027f17c5a4d807fe558c5651578eb4f72038408ca45da26f19066f74c656542cf161507eee2\
                       037db5882ae08f54b0f66c742e3a79375f49b5fc9ff01c2ddd8571b7c479e85b3f9abf5d26a69921ab2cc9728007588d69\
                       b2f90b0fe516ffb6be22c41658398ff87550dd01894fe7cf80b1bda6558c622da12fb2a5b57b1f60482a891f0a1430db3f\
                       748ce4124af7ef20ee6ff9b94c1a352517c7975be916ea72887b9ad8660f737bda241d79f76db11c5235001800c8315497\
                       236f5373dea887756f63ebc5487c07a40cb4c086e9b08c2130229d3740d5610745636296b01eabd0f1265f1278cacb8745\
                       0f715e291ccd4de1e497e88f0a068658b7aa8b1df29c17c37f83d1fbd8d9b82bb36d12c607087ebeda871000d879d64cbd\
                       2cd44468ba0e19ddb5dad885f0fed06d1c074868c08229f7269ea8a9d7d4e7de243333a4fe829e8b8a19123013defa5b00\
                       097611c1da36ad481631c58756ec4a6e4850ee00f1801949279d8fc043b589dca9a61edf02c2e707990a9c22b196e2b982\
                       f0af96aa87f3a0a38cf46d78be5b8ec18f6b7339a20ee75adf339c2d68f71478b28d06687f9deca7124180ecce34771716\
                       75f9d4e6e779016ede8e63ef4396fd7ad4318ed5561290ca7dcd2288a2ae8894c9c7339e8186681100").unwrap();

            // e(sa, b) = e(sb, a)
            // e(sa, b) * e(-sb, a) = 1
            assert!(Bls12_381::pairings(&input[..]).expect("pairings failed"));
            println!("test pairings{} success!", i + 1);
        }

        // check pairings
        {
            // hex![(a1, b1), (a2, b2), (-a1, b1), (-a2, b2)];
            let pairings_encoded = "bbc622db0af03afbef1a7af93fe8556c58ac1b173f3a4ea105b974974f8c68c30faca94f8c63952694d79731a7d3f117e1\
                       e7c5462923aa0ce48a88a244c73cd0edb3042ccb18db00f60ad0d595e0f5fce48a1d74ed309ea0f1a0aae381f4b30800b8\
                       bd21c1c85680d4efbb05a82603ac0b77d1e37a640b51b4023b40fad47ae4c65110c52d27050826910a8ff0b2a24a027e2b\
                       045d057dace5575d941312f14c3349507fdcbb61dab51ab62099d0d06b59654f2788a0d3ac7d609f7152602be0130128b8\
                       08865493e189a2ac3bccc93a922cd16051699a426da7d3bd8caa9bfdad1a352edac6cdc98c116e7d7227d5e50cbe795ff0\
                       5f07a9aaa11dec5c270d373fab992e57ab927426af63a7857e283ecb998bc22bb0d2ac32cc34a72ea0c4060600a99a8987\
                       b00903e92d4d1209ce7dc310915d801ec2414a4e571b9080ca8889065419a6f64bd90da2dfbb53514190ec09d4cecc58e6\
                       86ed79717f20afea668b9a8a6023927a6c94ab28e9f45e6cc830e4392c69132144f982704f7d4964e4430400d9e64f510e\
                       959d98fb0c2b7064f6fc21e90d4cb513a9dbb001cddd3d4ce4454527632d438d56a754971b17a25a59d70f1036ecbc7a5c\
                       8d0f27ba421c1c0a068cff756aa419cc42913257a5d78da12ef9194a11767e0e5d0f3739dbce4b21a90b611722a33ff03c\
                       02b91c54d72f6437b7fe9c27f4751bbd1ebdcfd0aa1898ef915ecdb2c9586f05e8faba135d0b64fd0d3610a6e94eb220c3\
                       523519374e582f03d08a8a51f7e452b7a3b7bf6cc8492e01b8c7b79b8929157a18b51ed5c3e26c0a00bbc622db0af03afb\
                       ef1a7af93fe8556c58ac1b173f3a4ea105b974974f8c68c30faca94f8c63952694d79731a7d3f117cac239b9d6dc54ad1b\
                       75cb0eba386f4e3642accad5b95566c907b51def6a8167f2212ecfc8767daaa845d555681d4d1100b8bd21c1c85680d4ef\
                       bb05a82603ac0b77d1e37a640b51b4023b40fad47ae4c65110c52d27050826910a8ff0b2a24a027e2b045d057dace5575d\
                       941312f14c3349507fdcbb61dab51ab62099d0d06b59654f2788a0d3ac7d609f7152602be0130128b808865493e189a2ac\
                       3bccc93a922cd16051699a426da7d3bd8caa9bfdad1a352edac6cdc98c116e7d7227d5e50cbe795ff05f07a9aaa11dec5c\
                       270d373fab992e57ab927426af63a7857e283ecb998bc22bb0d2ac32cc34a72ea0c4060600a99a8987b00903e92d4d1209\
                       ce7dc310915d801ec2414a4e571b9080ca8889065419a6f64bd90da2dfbb53514190ec09d7db32a7197911408e80330214\
                       99208499958d6426669cbb96299094188346809d80e22f956322c8299702f0852dbd1500d9e64f510e959d98fb0c2b7064\
                       f6fc21e90d4cb513a9dbb001cddd3d4ce4454527632d438d56a754971b17a25a59d70f1036ecbc7a5c8d0f27ba421c1c0a\
                       068cff756aa419cc42913257a5d78da12ef9194a11767e0e5d0f3739dbce4b21a90b611722a33ff03c02b91c54d72f6437\
                       b7fe9c27f4751bbd1ebdcfd0aa1898ef915ecdb2c9586f05e8faba135d0b64fd0d3610a6e94eb220c3523519374e582f03\
                       d08a8a51f7e452b7a3b7bf6cc8492e01b8c7b79b8929157a18b51ed5c3e26c0a00";

            let input: Vec<u8> = FromHex::from_hex(pairings_encoded).unwrap();

            // check pairings operation:(a1*b1) * e(a2*b2) * e(-a1*b1) * e(-a2*b2) == 1 return true
            assert!(Bls12_381::pairings(&input[..]).unwrap());
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

    let ans1 = Bls12_381::pairing(sa, b);
    let ans2 = Bls12_381::pairing(a, sb);
    let ans3 = Bls12_381::pairing(a, b).pow(s.into_repr());

    assert_eq!(ans1, ans2);
    assert_eq!(ans2, ans3);

    assert_ne!(ans1, Fq12::one());
    assert_ne!(ans2, Fq12::one());
    assert_ne!(ans3, Fq12::one());

    assert_eq!(ans1.pow(Fr::characteristic()), Fq12::one());
    assert_eq!(ans2.pow(Fr::characteristic()), Fq12::one());
    assert_eq!(ans3.pow(Fr::characteristic()), Fq12::one());
}
