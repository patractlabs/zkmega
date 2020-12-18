//! Verify
#![allow(unused)]
use core::convert::TryInto;
use matter_labs::{add, mul, pairing, result::Result, Bls12, Bn256};
use num_bigint::BigUint;
use num_traits::Num;

static BN: usize = 32;
static BLS: usize = 48;

static BN256_SCALAR_FIELD: &'static str =
    "21888242871839275222246405745257275088548364400416034343698204186575808495617";

static BN256_MODULUS: &'static str =
    "21888242871839275222246405745257275088696311157297823662689037894645226208583";

static BLS381_SCALAR_FIELD: &'static str =
    "52435875175126190479447740508185965837690552500527637822603658699938581184513";

static BLS381_MODULUS: &'static str =
    "4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787";

fn negate_y_based_curve(y: BigUint, MODULUS: &'static str) -> Result<BigUint> {
    let q = BigUint::from_str_radix(MODULUS, 10).unwrap();
    let q_clone = q.clone();
    Ok(q - y % q_clone)
}

fn negate_y(y: &[u8]) -> Result<Vec<u8>> {
    let negate_y = BigUint::from_bytes_be(y);
    let neg_y = match y.len() {
        32 => negate_y_based_curve(negate_y, BN256_MODULUS)?.to_bytes_be(),
        48 => negate_y_based_curve(negate_y, BLS381_MODULUS)?.to_bytes_be(),
        // _ => return Err(Megaclite("Invalid y coordinate length!".to_string())),
        _ => return Ok(vec![]),
    };
    // Because of randomness, Negate_y vector might not satisfy 32 or 48 bytes.
    let mut neg_y_fill_with_zero = vec![0u8; y.len()];
    if neg_y.len() != y.len() {
        neg_y_fill_with_zero[y.len() - neg_y.len()..y.len()].copy_from_slice(&*neg_y);
    } else {
        neg_y_fill_with_zero[0..y.len()].copy_from_slice(&*neg_y);
    }
    Ok(neg_y_fill_with_zero)
}

pub fn bls381_verify_proof(
    vk_gamma_abc: &[&[u8]],
    vk: &[u8],
    proof: &[u8],
    public_inputs: &[&[u8]],
) -> Result<bool> {
    if (public_inputs.len() + 1) != vk_gamma_abc.len() {
        // return Err(Megaclite("verifying key was malformed.".to_string()));
        return Ok(false);
    }

    // First two fields are used as the sum
    let mut acc: [u8; 48 * 2] = vk_gamma_abc[0].try_into().unwrap();

    // Compute the linear combination vk_x
    //  [(βui(x)+αvi(x)+wi(x))/γ] ∈ G1
    // acc = sigma(i:0~l)* [(βui(x)+αvi(x)+wi(x))/γ] ∈ G1
    for (i, b) in public_inputs.iter().zip(vk_gamma_abc.iter().skip(1)) {
        if BigUint::from_bytes_le(i)
            >= BigUint::from_str_radix(BLS381_SCALAR_FIELD, 10).expect("wrong ")
        {
            // return Err(Megaclite("Invalid public input!".to_string()));
            return Ok(false);
        }
        let mut mul_res = Vec::new();
        mul_res.extend_from_slice(b);
        mul_res.extend_from_slice(i);

        let mul_ic = mul::<Bls12>(&*mul_res)?;

        let mut acc_mul_ic = Vec::new();
        acc_mul_ic.extend_from_slice(&acc);
        acc_mul_ic.extend_from_slice(&mul_ic);

        acc = add::<Bls12>(&*acc_mul_ic)?;
    }

    // The original verification equation is:
    // A * B = alpha * beta + acc * gamma + C * delta
    // ... however, we rearrange it so that it is:
    // A * B - acc * gamma - C * delta = alpha * beta
    // or equivalently:
    //    A   *    B    +  (-acc) * gamma +  (-C) * delta  +   (-alpha) * beta = 0
    // [(g1_x, g1_y0, g2),(g1_x, g1_y0, g2),(g1_x, g1_y0, g2), (g1_x, g1_y0, g2)]
    let pairings = [
        (
            &proof[0..BLS],
            &proof[BLS..BLS * 2],
            &proof[BLS * 2..BLS * 6],
        ),
        (
            &acc[0..BLS],
            &*negate_y(&acc[BLS..BLS * 2])?,
            &vk[0..BLS * 4],
        ),
        (
            &proof[BLS * 6..BLS * 7],
            &*negate_y(&proof[BLS * 7..BLS * 8])?,
            &vk[BLS * 4..BLS * 8],
        ),
        (
            &vk[BLS * 8..BLS * 9],
            &*negate_y(&vk[BLS * 9..BLS * 10])?,
            &vk[BLS * 10..BLS * 14],
        ),
    ];

    let mut input = vec![0u8; BLS * 6 * 4];
    pairings.iter().enumerate().for_each(|(i, (x, y, g2))| {
        input[6 * i * BLS..(6 * i + 1) * BLS].copy_from_slice(x);
        input[(6 * i + 1) * BLS..(6 * i + 2) * BLS].copy_from_slice(y);
        input[(6 * i + 2) * BLS..(6 * i + 6) * BLS].copy_from_slice(g2);
    });

    // Return the result of computing the pairing check
    // e(p1[0], p2[0]) *  .... * e(p1[n], p2[n]) == 1.
    // For example pairing([P1(), P1().negate()], [P2(), P2()]) should return true.
    pairing::<Bls12>(&input[..])
}

pub fn bn256_verify_proof(
    vk_gamma_abc: &[&[u8]],
    vk: &[u8],
    proof: &[u8],
    public_inputs: &[&[u8]],
) -> Result<bool> {
    if (public_inputs.len() + 1) != vk_gamma_abc.len() {
        // return Err(Megaclite("verifying key was malformed.".to_string()));
        return Ok(false);
    }

    // First two fields are used as the sum
    let mut acc: [u8; 32 * 2] = vk_gamma_abc[0].try_into().unwrap();

    // Compute the linear combination vk_x
    //  [(βui(x)+αvi(x)+wi(x))/γ] ∈ G1
    // acc = sigma(i:0~l)* [(βui(x)+αvi(x)+wi(x))/γ] ∈ G1
    for (i, b) in public_inputs.iter().zip(vk_gamma_abc.iter().skip(1)) {
        if BigUint::from_bytes_be(i) >= BigUint::from_str_radix(BN256_SCALAR_FIELD, 10).unwrap() {
            // return Err(Megaclite("Invalid public input!".to_string()));
            return Ok(false);
        }
        let mut mul_res = vec![0u8; BN * 3];
        mul_res[0..BN * 2].copy_from_slice(b);
        mul_res[BN * 2..BN * 3].copy_from_slice(i);

        let mul_ic = mul::<Bn256>(&mul_res)?;

        let mut acc_mul_ic = vec![0u8; BN * 4];
        acc_mul_ic[0..BN * 2].copy_from_slice(&acc);
        acc_mul_ic[BN * 2..BN * 4].copy_from_slice(&mul_ic);

        acc = add::<Bn256>(&acc_mul_ic)?;
    }

    // The original verification equation is:
    // A * B = alpha * beta + acc * gamma + C * delta
    // ... however, we rearrange it so that it is:
    // A * B - acc * gamma - C * delta = alpha * beta
    // or equivalently:
    // A * B + (-acc) * gamma + (-C) * delta + (-alpha) * beta = 0
    // which allows us to do a single final exponentiation.

    let pairings = [
        (&proof[0..BN], &proof[BN..BN * 2], &proof[BN * 2..BN * 6]),
        (&acc[0..BN], &*negate_y(&acc[BN..BN * 2])?, &vk[0..BN * 4]),
        (
            &proof[BN * 6..BN * 7],
            &*negate_y(&proof[BN * 7..BN * 8])?,
            &vk[BN * 4..BN * 8],
        ),
        (
            &vk[BN * 8..BN * 9],
            &*negate_y(&vk[BN * 9..BN * 10])?,
            &vk[BN * 10..BN * 14],
        ),
    ];

    let mut input = vec![0u8; BN * 6 * 4];
    pairings.iter().enumerate().for_each(|(i, (x, y, g2))| {
        input[6 * i * BN..(6 * i + 1) * BN].copy_from_slice(x);
        input[(6 * i + 1) * BN..(6 * i + 2) * BN].copy_from_slice(y);
        input[(6 * i + 2) * BN..(6 * i + 6) * BN].copy_from_slice(g2);
    });

    // Return the result of computing the pairing check
    // e(p1[0], p2[0]) *  .... * e(p1[n], p2[n]) == 1.
    // For example pairing([P1(), P1().negate()], [P2(), P2()]) should return true.
    pairing::<Bn256>(&input[..])
}

#[test]
fn test_base_point_addition_and_doubling() {
    use bellman_ce::pairing::{
        bn256::{Fr, G1},
        ff::{PrimeField, PrimeFieldRepr},
        CurveAffine, CurveProjective,
    };
    let a = G1::one().into_affine().into_uncompressed();
    let b = G1::zero().into_affine().into_uncompressed();
    let mut input = [0u8; 128];
    input[0..64].copy_from_slice(a.as_ref());
    input[64..].copy_from_slice(b.as_ref());

    let scalar = Fr::from_str("2").unwrap();
    let mut input2 = [0u8; 96];
    input2[0..64].copy_from_slice(a.as_ref());
    scalar.into_repr().write_be(&mut input2[64..]).unwrap();

    assert_eq!(a.as_ref(), add::<Bn256>(&input).unwrap().as_ref());
    // assert_eq!(mul(&input2).unwrap(), add(&input).unwrap());
}
