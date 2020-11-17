use bn::{arith::U256, pairing_batch, AffineG1, AffineG2, Fq, Fq2, Group, Gt, G1, G2};
use std::io::{self, Read};

// Can fail if any of the 2 points does not belong the bn128 curve
pub fn alt_bn128_add(input: &[u8]) -> Result<[u8; 64], &'static str> {
    let mut padded_input = input.chain(io::repeat(0));
    let p1 = read_point(&mut padded_input)?;
    let p2 = read_point(&mut padded_input)?;

    let mut write_buf = [0u8; 64];
    if let Some(sum) = AffineG1::from_jacobian(p1 + p2) {
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
    use parity_bytes::BytesRef;

    // zero-points additions
    {
        let input = hex!(
            "
				0000000000000000000000000000000000000000000000000000000000000000
				0000000000000000000000000000000000000000000000000000000000000000
				0000000000000000000000000000000000000000000000000000000000000000
				0000000000000000000000000000000000000000000000000000000000000000"
        );

        let mut output = vec![0u8; 64];
        let expected = hex!(
            "
				0000000000000000000000000000000000000000000000000000000000000000
				0000000000000000000000000000000000000000000000000000000000000000"
        );

        f.execute(&input[..], &mut BytesRef::Fixed(&mut output[..]))
            .expect("Builtin should not fail");
        assert_eq!(output, &expected[..]);
    }

    // no input, should not fail
    {
        let mut empty = [0u8; 0];
        let input = BytesRef::Fixed(&mut empty);

        let mut output = vec![0u8; 64];
        let expected = hex!(
            "
				0000000000000000000000000000000000000000000000000000000000000000
				0000000000000000000000000000000000000000000000000000000000000000"
        );

        f.execute(&input[..], &mut BytesRef::Fixed(&mut output[..]))
            .expect("Builtin should not fail");
        assert_eq!(output, &expected[..]);
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

        let res = f.execute(&input[..], &mut BytesRef::Fixed(&mut output[..]));
        assert!(res.is_err(), "There should be built-in error here");
    }
}
#[test]
fn test_alt_bn128_mul() {
    use hex_literal::hex;
    use parity_bytes::BytesRef;

    // zero-point multiplication
    {
        let input = hex!(
            "
				0000000000000000000000000000000000000000000000000000000000000000
				0000000000000000000000000000000000000000000000000000000000000000
				0200000000000000000000000000000000000000000000000000000000000000"
        );

        let mut output = vec![0u8; 64];
        let expected = hex!(
            "
				0000000000000000000000000000000000000000000000000000000000000000
				0000000000000000000000000000000000000000000000000000000000000000"
        );

        f.execute(&input[..], &mut BytesRef::Fixed(&mut output[..]))
            .expect("Builtin should not fail");
        assert_eq!(output, &expected[..]);
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

        let res = f.execute(&input[..], &mut BytesRef::Fixed(&mut output[..]));
        assert!(res.is_err(), "There should be built-in error here");
    }
}

#[test]
fn add() {
    use bn::{Fr, Group, G1};
    use hex_literal::hex;
    use rand::{rngs::StdRng, SeedableRng};

    let seed = [
        0, 0, 0, 0, 0, 0, 64, 13, // 103245
        0, 0, 0, 0, 0, 0, 176, 2, // 191922
        0, 0, 0, 0, 0, 0, 0, 13, // 1293
        0, 0, 0, 0, 0, 0, 96, 7u8, // 192103
    ];

    let p1 = G1::random(&mut StdRng::from_seed(seed));

    println!("p1:{:?}", p1);
    println!("p1 + p1:{:?}", p1 + p1);
    println!("p1 * 2: {:?}", p1 * Fr::from_str("2").unwrap());

    let x = U256([
        193057075356696845107056778628997597259,
        58302148128148680438974674437016463407,
    ]);
    let y = U256([
        97531101576300296613937243006504225138,
        35484485583128177644927852191793807725,
    ]);
    let z = U256([
        173294647386577742778337618982106380048,
        54444350454913557390070431662659890840,
    ]);
    let p1_2times = alt_bn128_scalar_mul(
        &hex!("0230644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd46")[..],
    );
    println!("{:?}", p1_2times);
}
