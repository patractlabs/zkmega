use bls::{multi_miller_loop, G1Affine, G1Projective, G2Affine, G2Prepared, Gt, Scalar};
use core::convert::{TryFrom, TryInto};

// Point add on bls12_381 curve, return a point.
pub fn bls381_add(point1: &[u8], point2: &[u8]) -> Result<[u8; 96], &'static str> {
    if point1.len() != 96 || point2.len() != 96 {
        return Err("Invalid input length, should be 96 length slice(uncompressed)");
    }

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
pub fn bls381_scalar_mul(point: &[u8], scalar: &[u8]) -> Result<[u8; 96], &'static str> {
    if point.len() != 96 && scalar.len() != 32 {
        return Err("point or scalar Invalid input length, should be 96(uncompressed point) or 32(scalar) length slice");
    }

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
pub fn bls381_pairing(input: &[u8]) -> Result<bool, &'static str> {
    if input.len() % 288 != 0 {
        return Err("Invalid input length, must be multiple of 288 (3 * (48*2)) (uncompressed)");
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
    Ok(ret_val == Gt::identity())
}

#[test]
fn test_bls381_add() {
    use rustc_hex::{FromHex, ToHex};

    // Test identity add.
    {
        let a_hex = "400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
        let a_uncompressed: Vec<u8> = a_hex.from_hex().unwrap();
        let a_hex = "400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
        let a_uncompressed: Vec<u8> = a_hex.from_hex().unwrap();

        let c_uncompressed = bls381_add(a_uncompressed.as_ref(), a_uncompressed.as_ref())
            .expect("identity add failed");

        let c = G1Affine::from_uncompressed(&c_uncompressed).unwrap();
        assert!(bool::from(c.is_identity()));
        assert!(bool::from(c.is_on_curve()));
    }
    {
        let p1_hex = "17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb08b3f481e3aaa0f1a09e30ed741d8ae4fcf5e095d5d00af600db18cb2c04b3edd03cc744a2888ae40caa232946c5e7e1";
        let p1_uncompressed: Vec<u8> = p1_hex.from_hex().unwrap();

        let p1_add_p1 = bls381_add(&p1_uncompressed[..], &p1_uncompressed[..]).expect("add fail:");

        let p2_hex = "0572cbea904d67468808c8eb50a9450c9721db309128012543902d0ac358a62ae28f75bb8f1c7c42c39a8c5529bf0f4e166a9d8cabc673a322fda673779d8e3822ba3ecb8670e461f73bb9021d5fd76a4c56d9d4cd16bd1bba86881979749d28";
        let p2_uncompressed: Vec<u8> = p2_hex.from_hex().unwrap();
        assert_eq!(p2_uncompressed, p1_add_p1.to_vec());
    }
}
#[test]
fn test_bls381_mul() {}
#[test]
fn test_bls381_pairing() {}
