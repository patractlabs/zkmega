use bls12_381::{ G1Affine, G2Affine,G2Prepared,G1Projective, Scalar,Gt, multi_miller_loop};
use std::convert::TryFrom;

pub fn bls381_add(point1: &[u8], point2: &[u8]) -> Result<[u8;96], &'static str> {
    if point1.len()!=96 || point2.len()!=96 {
        return Err("Invalid input length, should be 96 length slice(uncompressed)");
    }

    let p1 = G1Affine::from_uncompressed (
        <&[u8; 96]>::try_from(point1)
            .map_err(|_| "point1 slice try_from &[u8;64] fail")?
    );
    let p1 :G1Affine =  Option::from(p1).ok_or("Invalid a pairing G1Affine")?;
    let p2 = G1Affine::from_uncompressed (
        <&[u8; 96]>::try_from(point2)
            .map_err(|_| "point2 slice try_from &[u8;64] fail")?
    );
    let p2 :G1Affine =  Option::from(p2).ok_or( "Invalid a pairing G1Affine")?;

    let add_res = G1Affine::from( p1 +  G1Projective::from(p2));
    Ok(add_res.to_uncompressed())
}

pub fn bls381_scalar_mul(point: &[u8], scalar: u64) -> Result<[u8;96], &'static str> {
    if point.len()!=96 {
        return Err("Invalid input length, should be 96 length slice(uncompressed)");
    }

    let p = G1Affine::from_uncompressed (
        <&[u8; 96]>::try_from(point)
            .map_err(|_| "point1 slice try_from &[u8;64] fail")?
    );
    let p :G1Affine =  Option::from(p).ok_or("Invalid a pairing G1Affine")?;
    let scalar = Scalar::from(scalar);

    let mul_res = G1Affine::from(p * scalar);
    Ok(mul_res.to_uncompressed())
}

pub fn bls381_pairing(input: &[u8]) -> Result<bool, &'static str> {
    if input.len() % 289 != 0 {
        return Err("Invalid input length, must be multiple of 289 (3 * (48*2)) (uncompressed)")
    }

    let ret_val = if input.is_empty() {
        Gt::identity()
    } else {
        let elements = input.len() /  289;
        let mut vals = Vec::new();
        for idx in 0..elements {
            let a = G1Affine::from_uncompressed (
                <&[u8; 96]>::try_from(&input[idx * 289..idx * 289 + 96])
                    .map_err(|_| "point1 slice try_from &[u8;64] fail")?
            );
            let a :G1Affine =  Option::from(a).ok_or("Invalid a pairing G1Affine")?;
            let b = G2Affine::from_uncompressed (
                <&[u8; 192]>::try_from(&input[idx * 289 + 96..idx * 289 + 289])
                    .map_err(|_| "point1 slice try_from &[u8;64] fail")?
            );
            let b :G2Affine =  Option::from(b).ok_or("Invalid a pairing G1Affine")?;

            vals.push((a , G2Prepared::from(b)));
        };

        multi_miller_loop(
            &vals.iter().map(
                |point|(&point.0,&point.1)
            )
                .collect::<Vec<_>>()
        ).final_exponentiation()
    };
    Ok(ret_val==Gt::identity())
}

#[test]
fn test_bls381_add() {
    // use bls12_381::Fq;
    // {
    //     let a = G1Affine::identity();
    //     let b = G1Projective::identity();
    //     let c = a + b;
    //     assert!(bool::from(c.is_identity()));
    //     assert!(bool::from(c.is_on_curve()));
    // }
    // {
    //     let a = G1Affine::identity();
    //     let mut b = G1Projective::generator();
    //     {
    //         let z = Fp::from_raw_unchecked([
    //             0xba7a_fa1f_9a6f_e250,
    //             0xfa0f_5b59_5eaf_e731,
    //             0x3bdc_4776_94c3_06e7,
    //             0x2149_be4b_3949_fa24,
    //             0x64aa_6e06_49b2_078c,
    //             0x12b1_08ac_3364_3c3e,
    //         ]);
    //
    //         b = G1Projective {
    //             x: b.x * z,
    //             y: b.y * z,
    //             z,
    //         };
    //     }
    //     let c = a + b;
    //     assert!(!bool::from(c.is_identity()));
    //     assert!(bool::from(c.is_on_curve()));
    //     assert!(c == G1Projective::generator());
    // }
    // {
    //     let a = G1Affine::identity();
    //     let mut b = G1Projective::generator();
    //     {
    //         let z = Fp::from_raw_unchecked([
    //             0xba7a_fa1f_9a6f_e250,
    //             0xfa0f_5b59_5eaf_e731,
    //             0x3bdc_4776_94c3_06e7,
    //             0x2149_be4b_3949_fa24,
    //             0x64aa_6e06_49b2_078c,
    //             0x12b1_08ac_3364_3c3e,
    //         ]);
    //
    //         b = G1Projective {
    //             x: b.x * z,
    //             y: b.y * z,
    //             z,
    //         };
    //     }
    //     let c = b + a;
    //     assert!(!bool::from(c.is_identity()));
    //     assert!(bool::from(c.is_on_curve()));
    //     assert!(c == G1Projective::generator());
    // }
    // {
    //     let a = G1Projective::generator().double().double(); // 4P
    //     let b = G1Projective::generator().double(); // 2P
    //     let c = a + b;
    //
    //     let mut d = G1Projective::generator();
    //     for _ in 0..5 {
    //         d += G1Affine::generator();
    //     }
    //     assert!(!bool::from(c.is_identity()));
    //     assert!(bool::from(c.is_on_curve()));
    //     assert!(!bool::from(d.is_identity()));
    //     assert!(bool::from(d.is_on_curve()));
    //     assert_eq!(c, d);
    // }
    //
    // // Degenerate case
    // {
    //     let beta = Fp::from_raw_unchecked([
    //         0xcd03_c9e4_8671_f071,
    //         0x5dab_2246_1fcd_a5d2,
    //         0x5870_42af_d385_1b95,
    //         0x8eb6_0ebe_01ba_cb9e,
    //         0x03f9_7d6e_83d0_50d2,
    //         0x18f0_2065_5463_8741,
    //     ]);
    //     let beta = beta.square();
    //     let a = G1Projective::generator().double().double();
    //     let b = G1Projective {
    //         x: a.x * beta,
    //         y: -a.y,
    //         z: a.z,
    //     };
    //     let a = G1Affine::from(a);
    //     assert!(bool::from(a.is_on_curve()));
    //     assert!(bool::from(b.is_on_curve()));
    //
    //     let c = a + b;
    //     assert_eq!(
    //         G1Affine::from(c),
    //         G1Affine::from(G1Projective {
    //             x: Fp::from([
    //                 0x29e1_e987_ef68_f2d0,
    //                 0xc5f3_ec53_1db0_3233,
    //                 0xacd6_c4b6_ca19_730f,
    //                 0x18ad_9e82_7bc2_bab7,
    //                 0x46e3_b2c5_785c_c7a9,
    //                 0x07e5_71d4_2d22_ddd6,
    //             ]),
    //             y: Fp::from_raw_unchecked([
    //                 0x94d1_17a7_e5a5_39e7,
    //                 0x8e17_ef67_3d4b_5d22,
    //                 0x9d74_6aaf_508a_33ea,
    //                 0x8c6d_883d_2516_c9a2,
    //                 0x0bc3_b8d5_fb04_47f7,
    //                 0x07bf_a4c7_210f_4f44,
    //             ]),
    //             z: Fp::one()
    //         })
    //     );
    //     assert!(!bool::from(c.is_identity()));
    //     assert!(bool::from(c.is_on_curve()));
    // }
}
#[test]
fn test_bls381_mul() {

}
#[test]
fn test_bls381_pairing() {

}