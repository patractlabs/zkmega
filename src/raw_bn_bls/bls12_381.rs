use super::Curve;
use bellman_ce::pairing::bls12_381::Bls12;
use bls::{multi_miller_loop, G1Affine, G1Projective, G2Affine, G2Prepared, Gt, Scalar};
use core::convert::TryFrom;

impl<'a> Curve<'a> for Bls12 {
    // curve parameters
    const SCALAR_FIELD: &'static str =
        "52435875175126190479447740508185965837690552500527637822603658699938581184513";
    const PRIME_FIELD: &'static str =
        "4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787";
    const FQ_BYTES_LENGTH: usize = 48;

    type Point = [u8; 96];

    // Point add on bls12_381 curve, return a point.
    fn point_add(input: &[u8]) -> Result<Self::Point, &'static str> {
        if input.len() != 96 * 2 {
            return Err("Invalid input length, should be 96*2 length slice(uncompressed)");
        }
        let (point1, point2) = input.split_at(96);

        let p1 = G1Affine::from_uncompressed(
            <&[u8; 96]>::try_from(point1).map_err(|_| "point1 slice try_from &[u8;64] fail")?,
        );
        let p1: G1Affine = Option::from(p1).ok_or("Invalid a pairing G1Affine")?;
        let p2 = G1Affine::from_uncompressed(
            <&[u8; 96]>::try_from(point2).map_err(|_| "point2 slice try_from &[u8;64] fail")?,
        );
        let p2: G1Affine = Option::from(p2).ok_or("Invalid a pairing G1Affine")?;

        let add_res = G1Affine::from(p1 + G1Projective::from(p2));
        Ok(add_res.to_uncompressed())
    }

    // Scalar mul point on bls12_381 curve, return a point.
    fn point_scalar_mul(input: &[u8]) -> Result<Self::Point, &'static str> {
        if input.len() != 96 + 32 {
            return Err("point or scalar Invalid input length, should be 128 = 96(uncompressed point) + 32(scalar) length slice");
        }
        let (point, scalar) = input.split_at(96);

        let p = G1Affine::from_uncompressed(
            <&[u8; 96]>::try_from(point).map_err(|_| "point1 slice try_from &[u8;64] fail")?,
        );
        let p: G1Affine = Option::from(p).ok_or("Invalid a pairing G1Affine")?;

        let scalar = Scalar::from_bytes(<&[u8; 32]>::try_from(scalar).unwrap());
        let scalar: Scalar = Option::from(scalar).ok_or("Invalid a pairing G1Affine")?;

        let mul_res = G1Affine::from(p * scalar);
        Ok(mul_res.to_uncompressed())
    }

    // Return the result of computing the pairing check
    // e(p1[0], p2[0]) *  .... * e(p1[n], p2[n]) == 1.
    // For example pairing([P1(), P1().negate()], [P2(), P2()]) should return true.
    fn point_pairing(input: &[u8]) -> Result<bool, &'static str> {
        if input.len() % 288 != 0 {
            return Err(
                "Invalid input length, must be multiple of 288 (3 * (48*2)) (uncompressed)",
            );
        }

        let ret_val = if input.is_empty() {
            Gt::identity()
        } else {
            let elements = input.len() / 288;
            let mut vals = Vec::new();
            for idx in 0..elements {
                let a = G1Affine::from_uncompressed(
                    <&[u8; 96]>::try_from(&input[idx * 288..idx * 288 + 96])
                        .map_err(|_| "point1 slice try_from &[u8;64] fail")?,
                );
                let a: G1Affine = Option::from(a).ok_or("Invalid a pairing G1Affine")?;
                let b = G2Affine::from_uncompressed(
                    <&[u8; 192]>::try_from(&input[idx * 288 + 96..idx * 288 + 288])
                        .map_err(|_| "point1 slice try_from &[u8;64] fail")?,
                );
                let b: G2Affine = Option::from(b).ok_or("Invalid a pairing G1Affine")?;

                vals.push((a, G2Prepared::from(b)));
            }

            multi_miller_loop(
                &vals
                    .iter()
                    .map(|point| (&point.0, &point.1))
                    .collect::<Vec<_>>(),
            )
            .final_exponentiation()
        };
        // assert_eq!(ret_val, Gt::identity());
        Ok(ret_val == Gt::identity())
    }
}

#[test]
fn test_bls381_add() {
    use rustc_hex::FromHex;

    // Test identity add.
    {
        let a_hex = "400000000000000000000000000000000000000000000000\
                            000000000000000000000000000000000000000000000000\
                            000000000000000000000000000000000000000000000000\
                            000000000000000000000000000000000000000000000000";
        let a_uncompressed: Vec<u8> = a_hex.from_hex().unwrap();

        let c_uncompressed =
            Bls12::point_add(a_uncompressed.repeat(2).as_ref()).expect("identity add failed");

        let c = G1Affine::from_uncompressed(&c_uncompressed).unwrap();
        assert!(bool::from(c.is_identity()));
        assert!(bool::from(c.is_on_curve()));
    }
    {
        let p1_hex = "17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905\
                             a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb\
                             08b3f481e3aaa0f1a09e30ed741d8ae4fcf5e095d5d00af6\
                             00db18cb2c04b3edd03cc744a2888ae40caa232946c5e7e1";
        let p1_uncompressed: Vec<u8> = p1_hex.from_hex().unwrap();

        let p1_add_p1 = Bls12::point_add(&p1_uncompressed.repeat(2)[..]).expect("add fail:");

        let p2_hex = "0572cbea904d67468808c8eb50a9450c9721db309128012543902d0ac358a62ae28f75bb8f1c7c42c39a8c5529bf0f4e166a9d8cabc673a322fda673779d8e3822ba3ecb8670e461f73bb9021d5fd76a4c56d9d4cd16bd1bba86881979749d28";
        let p2_uncompressed: Vec<u8> = p2_hex.from_hex().unwrap();
        assert_eq!(p2_uncompressed, p1_add_p1.to_vec());
    }
}

#[test]
fn test_bls381_mul() {
    let g = G1Affine::generator();

    let a = Scalar::from_raw([
        0x2b56_8297_a56d_a71c,
        0xd8c3_9ecb_0ef3_75d1,
        0x435c_38da_67bf_bf96,
        0x8088_a050_26b6_59b2,
    ]);

    let b = Scalar::from_raw([
        0x785f_dd9b_26ef_8b85,
        0xc997_f258_3769_5c18,
        0x4c8d_bc39_e7b7_56c1,
        0x70d9_b6cc_6d87_df20,
    ]);

    let c = a * b;

    let mut input = [0u8; 128];
    input[0..96].copy_from_slice(&g.to_uncompressed());
    input[96..128].copy_from_slice(&c.to_bytes());

    // (a·G)·b = (a * b)·G = c·G
    assert_eq!(
        G1Affine::from(G1Affine::from(g * a) * b).to_uncompressed(),
        Bls12::point_scalar_mul(&input).unwrap()
    );
    assert_eq!(G1Affine::from(g * a) * b, g * c);
}

#[test]
fn test_bls381_pairing() {
    use bls::pairing;
    use std::ops::Neg;

    let a1 = G1Affine::generator();
    let b1 = G2Affine::generator();

    let a2 = G1Affine::from(
        G1Affine::generator() * Scalar::from_raw([1, 2, 3, 4]).invert().unwrap().square(),
    );
    let b2 = G2Affine::from(
        G2Affine::generator() * Scalar::from_raw([4, 2, 2, 4]).invert().unwrap().square(),
    );

    // -a3
    let a3 = G1Affine::generator().neg();
    let b3 = G2Affine::generator();

    // -a4
    let a4 = G1Affine::from(
        G1Affine::generator() * Scalar::from_raw([1, 2, 3, 4]).invert().unwrap().square(),
    )
    .neg();
    let b4 = G2Affine::from(
        G2Affine::generator() * Scalar::from_raw([4, 2, 2, 4]).invert().unwrap().square(),
    );

    // a1 * b1  + a2 * b2  + -a1 * b1  + -a2 * b2 = 0
    let expected = pairing(&a1, &b1) + pairing(&a2, &b2) + pairing(&a3, &b3) + pairing(&a4, &b4);

    let pairings = [(a1, b1), (a2, b2), (a3, b3), (a4, b4)];

    let mut input = [0u8; 288 * 4];
    pairings.iter().enumerate().for_each(|(i, (a, b))| {
        input[96 * (3 * i)..96 * (3 * i + 1)].copy_from_slice(&a.to_uncompressed());
        input[96 * (3 * i + 1)..96 * (3 * i + 3)].copy_from_slice(&b.to_uncompressed());
    });

    // e(a1*b1) + e(a2*b2) + e(-a1*b1) + e(-a2*b2) = 1
    assert_eq!(Gt::identity(), expected);
    // check e(a1*b1) + e(a2*b2) + e(-a1*b1) + e(-a2*b2) == 1 return true
    assert!(Bls12::point_pairing(&input[..]).unwrap_or(false));
}
