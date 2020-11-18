use bn::{arith::U256, pairing_batch, AffineG1, AffineG2, Fq, Fq2, Group, Gt, G1, G2};
use rustc_hex::ToHex;
use std::io::{self, Read};

// Can fail if any of the 2 points does not belong the bn128 curve
pub fn alt_bn128_add(input: &[u8]) -> Result<[u8; 64], &'static str> {
    let mut padded_input = input.chain(io::repeat(0));
    let p1 = read_point(&mut padded_input)?;
    let p2 = read_point(&mut padded_input)?;

    let mut write_buf = [0u8; 64];
    if let Some(sum) = AffineG1::from_jacobian(p1 + p2) {
        // println!("sum in fn AffineG1:{:?}",sum);
        // point not at infinity
        sum.x()
            .to_big_endian(&mut write_buf[0..32])
            .expect("Cannot success since 0..32 is 32-byte length");
        sum.y()
            .to_big_endian(&mut write_buf[32..64])
            .expect("Cannot success since 32..64 is 32-byte length");
    }

    Ok(write_buf)
}

// Can fail if first parameter (bn128 curve point) does not actually belong to the curve
pub fn alt_bn128_scalar_mul(input: &[u8]) -> Result<[u8; 64], &'static str> {
    let mut padded_input = input.chain(io::repeat(0));
    let p = read_point(&mut padded_input)?;
    let fr = read_fr(&mut padded_input)?;

    let mut write_buf = [0u8; 64];
    if let Some(sum) = AffineG1::from_jacobian(p * fr) {
        // point not at infinity
        sum.x()
            .to_big_endian(&mut write_buf[0..32])
            .expect("Cannot fail since 0..32 is 32-byte length");
        sum.y()
            .to_big_endian(&mut write_buf[32..64])
            .expect("Cannot fail since 32..64 is 32-byte length");
    }
    Ok(write_buf)
}

pub fn alt_bn128_pairing(input: &[u8]) -> Result<bool, &'static str> {
    if input.len() % 192 != 0 {
        return Err("Invalid input length, must be multiple of 192 (3 * (32*2))");
    }

    let ret_val = if input.is_empty() {
        U256::one()
    } else {
        // (a, b_a, b_b - each 64-byte affine coordinates)
        let elements = input.len() / 192;
        let mut vals = Vec::new();
        for idx in 0..elements {
            let a_x = Fq::from_slice(&input[idx * 192..idx * 192 + 32])
                .map_err(|_| "Invalid a argument x coordinate")?;

            let a_y = Fq::from_slice(&input[idx * 192 + 32..idx * 192 + 64])
                .map_err(|_| "Invalid a argument y coordinate")?;

            let b_a_y = Fq::from_slice(&input[idx * 192 + 64..idx * 192 + 96])
                .map_err(|_| "Invalid b argument imaginary coeff x coordinate")?;

            let b_a_x = Fq::from_slice(&input[idx * 192 + 96..idx * 192 + 128])
                .map_err(|_| "Invalid b argument imaginary coeff y coordinate")?;

            let b_b_y = Fq::from_slice(&input[idx * 192 + 128..idx * 192 + 160])
                .map_err(|_| "Invalid b argument real coeff x coordinate")?;

            let b_b_x = Fq::from_slice(&input[idx * 192 + 160..idx * 192 + 192])
                .map_err(|_| "Invalid b argument real coeff y coordinate")?;

            let b_a = Fq2::new(b_a_x, b_a_y);
            let b_b = Fq2::new(b_b_x, b_b_y);
            let b = if b_a.is_zero() && b_b.is_zero() {
                G2::zero()
            } else {
                G2::from(AffineG2::new(b_a, b_b).map_err(|_| "Invalid b argument - not on curve")?)
            };
            let a = if a_x.is_zero() && a_y.is_zero() {
                G1::zero()
            } else {
                G1::from(AffineG1::new(a_x, a_y).map_err(|_| "Invalid a argument - not on curve")?)
            };
            vals.push((a, b));
        }

        let mul = pairing_batch(&vals);

        if mul == Gt::one() {
            U256::one()
        } else {
            U256::zero()
        }
    };

    Ok(ret_val == U256::one())
}

fn read_fr(reader: &mut io::Chain<&[u8], io::Repeat>) -> Result<bn::Fr, &'static str> {
    let mut buf = [0u8; 32];

    reader
        .read_exact(&mut buf[..])
        .expect("reading from zero-extended memory cannot fail; qed");
    // println!("scalar: {}",buf.to_hex::<String>());

    bn::Fr::from_slice(&buf[0..32]).map_err(|_| "Invalid field element")
}

fn read_point(reader: &mut io::Chain<&[u8], io::Repeat>) -> Result<bn::G1, &'static str> {
    let mut buf = [0u8; 32];

    reader
        .read_exact(&mut buf[..])
        .expect("reading from zero-extended memory cannot fail; qed");
    let px = Fq::from_slice(&buf[0..32]).map_err(|_| "Invalid point x coordinate")?;

    reader
        .read_exact(&mut buf[..])
        .expect("reading from zero-extended memory cannot fail; qed");
    let py = Fq::from_slice(&buf[0..32]).map_err(|_| "Invalid point y coordinate")?;
    println!("sum in fn AffineG1:{:?}", AffineG1::new(px, py).unwrap());
    Ok(if px == Fq::zero() && py == Fq::zero() {
        G1::zero()
    } else {
        AffineG1::new(px, py)
            .map_err(|_| "Invalid curve point")?
            .into()
    })
}

#[test]
fn test_alt_bn128_add() {
    use hex_literal::hex;
    use rustc_hex::FromHex;
    //two G1::one() additions
    {
        let mut p1_concat_p1:Vec<u8> = "0000000000000000000000000000000000000000000000000000000000000001\
                                      0000000000000000000000000000000000000000000000000000000000000002\
                                      0000000000000000000000000000000000000000000000000000000000000001\
                                      0000000000000000000000000000000000000000000000000000000000000002".from_hex().unwrap();

        let p1_add_p1: Vec<u8> = "030644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd3\
                                 15ed738c0e0a7c92e7845f96b2ae9c0a68a6a449e3538fc7ff3ebf7a5a18a2c4"
            .from_hex()
            .unwrap();

        assert_eq!(
            &alt_bn128_add(&*p1_concat_p1).expect("alt_bn128_add fail")[..],
            &p1_add_p1[..]
        );
    }
    // zero-points additions
    {
        let input = hex!(
            "
				0000000000000000000000000000000000000000000000000000000000000000
				0000000000000000000000000000000000000000000000000000000000000000
				0000000000000000000000000000000000000000000000000000000000000000
				0000000000000000000000000000000000000000000000000000000000000000"
        );

        let expected = hex!(
            "
				0000000000000000000000000000000000000000000000000000000000000000
				0000000000000000000000000000000000000000000000000000000000000000"
        );

        assert_eq!(
            &expected[..],
            alt_bn128_add(&input[..]).expect("Builtin should not fail")
        );
    }

    // no input, should not fail
    {
        let input = [0u8; 0];
        let expected = hex!(
            "
				0000000000000000000000000000000000000000000000000000000000000000
				0000000000000000000000000000000000000000000000000000000000000000"
        );

        assert_eq!(
            &expected[..],
            alt_bn128_add(&input[..]).expect("Builtin should not fail")
        );
    }

    // should fail - point not on curve
    {
        let input = hex!(
            "
				1111111111111111111111111111111111111111111111111111111111111111
				1111111111111111111111111111111111111111111111111111111111111111
				1111111111111111111111111111111111111111111111111111111111111111
				1111111111111111111111111111111111111111111111111111111111111111"
        );

        let mut output = vec![0u8; 64];

        let res = alt_bn128_add(&input[..]);
        assert!(res.is_err(), "There should be built-in error here");
    }
}
#[test]
fn test_alt_bn128_mul() {
    use hex_literal::hex;

    // zero-point multiplication
    {
        let input = hex!(
            "
				0000000000000000000000000000000000000000000000000000000000000000
				0000000000000000000000000000000000000000000000000000000000000000
				0200000000000000000000000000000000000000000000000000000000000000"
        );

        let expected = hex!(
            "
				0000000000000000000000000000000000000000000000000000000000000000
				0000000000000000000000000000000000000000000000000000000000000000"
        );

        assert_eq!(
            expected,
            alt_bn128_scalar_mul(&input[..]).expect("Builtin should not fail")
        );
    }

    // should fail - point not on curve
    {
        let input = hex!(
            "
				1111111111111111111111111111111111111111111111111111111111111111
				1111111111111111111111111111111111111111111111111111111111111111
				0f00000000000000000000000000000000000000000000000000000000000000"
        );

        let mut output = vec![0u8; 64];

        let res = alt_bn128_scalar_mul(&input[..]);
        assert!(res.is_err(), "There should be built-in error here");
    }
}

#[test]
fn test_alt_bn128_pairing() {
    use hex_literal::hex;
    // should not fail, because empty input is a valid input of 0 elements
    {
        let mut empty = [0u8; 0];
        let input = [0u8; 0];

        let mut expected = true;
        assert_eq!(
            expected,
            alt_bn128_pairing(&input[..]).expect("Builtin should not fail")
        );
    }
    {
        let input = &hex!(
            "
				1111111111111111111111111111111111111111111111111111111111111111
				1111111111111111111111111111111111111111111111111111111111111111
				1111111111111111111111111111111111111111111111111111111111111111
				1111111111111111111111111111111111111111111111111111111111111111
				1111111111111111111111111111111111111111111111111111111111111111
				1111111111111111111111111111111111111111111111111111111111111111"
        );
        let res = alt_bn128_pairing(input);

        if let Some(msg) = Some("not on curve") {
            if let Err(e) = res {
                if !e.contains(msg) {
                    panic!(
                        "There should be error containing '{}' here, but got: '{}'",
                        msg, e
                    );
                }
            }
        } else {
            assert!(res.is_err(), "There should be built-in error here");
        }
    }
    {
        let input = &hex!(
            "
				1111111111111111111111111111111111111111111111111111111111111111
				1111111111111111111111111111111111111111111111111111111111111111
				111111111111111111111111111111"
        );
        let res = alt_bn128_pairing(input);

        if let Some(msg) = Some("Invalid input length") {
            if let Err(e) = res {
                if !e.contains(msg) {
                    panic!(
                        "There should be error containing '{}' here, but got: '{}'",
                        msg, e
                    );
                }
            }
        } else {
            assert!(res.is_err(), "There should be built-in error here");
        }
    }
}

#[test]
fn add_all() {
    use bn::{AffineG1, Fq, Fr, Group, G1};
    use rustc_hex::FromHex;

    // let seed = [
    //     0, 0, 0, 0, 0, 0, 64, 13, // 103245
    //     0, 0, 0, 0, 0, 0, 176, 2, // 191922
    //     0, 0, 0, 0, 0, 0, 0, 13, // 1293
    //     0, 0, 0, 0, 0, 0, 96, 7u8, // 192103
    // ];
    // let mut p1 = G1::one();
    // let p1_affine  = AffineG1::from_jacobian(p1).unwrap();
    //
    // println!("p1:{:?}", p1);
    // println!("p1_affine:{:?}", AffineG1::from_jacobian(p1));
    // println!("p1.x:{:?}\np1.y:{:?}\np1.z:{:?}", p1.x(),p1.y(),p1.z());
    // println!("p1_affine.x:{:?} \np1_affine.y:{:?}\n", p1_affine.x(),p1_affine.y());
    //
    // let p1_sum = p1 + p1;
    // let p1_sum_affine = AffineG1::from_jacobian(p1_sum).unwrap();
    // let mut x_encoded = vec![0;32];
    // p1_sum_affine.x().to_big_endian(&mut x_encoded);
    // let mut y_encoded = vec![0;32];
    // p1_sum_affine.y().to_big_endian(&mut y_encoded);
    // println!("p1_sum_affine.x:{:?}\np1_sum_affine.y:{:?}\n", x_encoded.to_hex::<String>(),y_encoded.to_hex::<String>());
    //
    // println!("sum AffineG1:{:?}",  AffineG1::from_jacobian(p1 + p1).unwrap());
    // println!("p1_sum AffineG1:{:?}",  AffineG1::from_jacobian(p1_sum).unwrap());
    // let g1:G1=Into::into(AffineG1::from_jacobian(p1_sum).unwrap());
    // println!("p1_sum G1:{:?}", g1 );
    // let scalar = Fr::from_str("2").unwrap();
    // let p1_mul = p1 * scalar;
    // println!("p1*2 G1: {:?}", p1_mul);
    // println!("p1*2 AffineG1: {:?}\n", AffineG1::from_jacobian(p1_mul).unwrap());
    //
    // let x = p1_affine.x();
    // let y = p1_affine.y();
    //
    // let mut p1 = [0;64];
    // x.to_big_endian(&mut p1[0..32]);
    // y.to_big_endian(&mut p1[32..64]);
    //
    // let mut scalar_encoded = vec![0;32];
    // scalar.to_big_endian(&mut scalar_encoded);
    // println!("scalar: {}",scalar_encoded.to_hex::<String>());
    //
    // p1.extend_from_slice(&*x_encoded);
    // p1.extend_from_slice(&*y_encoded);

    // }

    // {
    //     let mut p1_concat_scalar = Vec::new();
    //     p1_concat_scalar.extend_from_slice(&p1);
    //     p1_concat_scalar.extend_from_slice(&scalar_encoded);
    //
    //     let mut p2 = alt_bn128_scalar_mul(&*p1_concat_scalar).expect("alt_bn128_mul fail");
    //     let mut padded_input = p2.split_at_mut(32);
    //     println!("p1*2 out fn G!: {:?}", Fq::from_slice(padded_input.0));
    // }
}
