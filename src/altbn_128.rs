use crate::*;
use crate::{
    result::Result,
    scratch::{Bytes, Curve},
};
use bellman_ce::pairing::{
    bn256::{Bn256, Fr, G1},
    ff::{PrimeField, PrimeFieldRepr},
    CurveAffine, CurveProjective,
};
use core::convert::TryInto;
use num_bigint::BigUint;
use num_traits::Num;

curve! {Bn256, 64, 128}

static BN: usize = 32;

pub fn bn256_verify_proof(
    vk_gammaABC: &[&[u8]],
    vk: &[u8],
    proof: &[u8],
    public_inputs: &[&[u8]],
) -> Result<bool> {
    if (public_inputs.len() + 1) != vk_gammaABC.len() {
        return Err(Megaclite("verifying key was malformed.".to_string()));
    }

    // First two fields are used as the sum
    let mut acc: [u8; 32 * 2] = vk_gammaABC[0].try_into()?;

    // Compute the linear combination vk_x
    //  [(βui(x)+αvi(x)+wi(x))/γ] ∈ G1
    // acc = sigma(i:0~l)* [(βui(x)+αvi(x)+wi(x))/γ] ∈ G1
    for (i, b) in public_inputs.iter().zip(vk_gammaABC.iter().skip(1)) {
        if BigUint::from_bytes_be(i) >= BigUint::from_str_radix(BN256_SCALAR_FIELD, 10)? {
            return Err(Megaclite("Invalid public input!".to_string()));
        }
        let mut mul_res = vec![0u8; BN * 3];
        mul_res[0..BN * 2].copy_from_slice(b);
        mul_res[BN * 2..BN * 3].copy_from_slice(i);

        let mul_ic = mul(&mul_res)?;

        let mut acc_mul_ic = vec![0u8; BN * 4];
        acc_mul_ic[0..BN * 2].copy_from_slice(&acc);
        acc_mul_ic[BN * 2..BN * 4].copy_from_slice(&mul_ic);

        acc = add(&acc_mul_ic)?;
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
    pairing(&input[..])
}

#[test]
fn test_base_point_addition_and_doubling() {
    let mut a = G1::one().into_affine().into_uncompressed();
    let mut b = G1::zero().into_affine().into_uncompressed();
    let mut input = [0u8; 128];
    input[0..64].copy_from_slice(a.as_ref());
    input[64..].copy_from_slice(b.as_ref());

    let scalar = Fr::from_str("2").unwrap();
    let mut input2 = [0u8; 96];
    input2[0..64].copy_from_slice(a.as_ref());
    scalar.into_repr().write_be(&mut input2[64..]);

    assert_eq!(a.as_ref(), add(&input).unwrap().as_ref());
    // assert_eq!(mul(&input2).unwrap(), add(&input).unwrap());
}
