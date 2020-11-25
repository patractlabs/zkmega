use crate::*;
use crate::{
    result::Result,
    scratch::{Bytes, Curve},
};
use bellman_ce::pairing::bls12_381::Bls12;
use core::convert::TryInto;

curve! {Bls12, 96, 192}

static BLS: usize = 48;

pub fn bls381_verify_proof(
    vk_gammaABC: &[&[u8]],
    vk: &[u8],
    proof: &[u8],
    public_inputs: &[&[u8]],
) -> Result<bool> {
    if (public_inputs.len() + 1) != vk_gammaABC.len() {
        return Err(Megaclite("verifying key was malformed.".to_string()));
    }

    // First two fields are used as the sum
    let mut acc: [u8; 48 * 2] = vk_gammaABC[0].try_into()?;

    // Compute the linear combination vk_x
    //  [(βui(x)+αvi(x)+wi(x))/γ] ∈ G1
    // acc = sigma(i:0~l)* [(βui(x)+αvi(x)+wi(x))/γ] ∈ G1
    for (i, b) in public_inputs.iter().zip(vk_gammaABC.iter().skip(1)) {
        if BigUint::from_bytes_le(i)
            >= BigUint::from_str_radix(BLS381_SCALAR_FIELD, 10).expect("wrong ")
        {
            return Err(Megaclite("Invalid public input!".to_string()));
        }
        let mut mul_res = Vec::new();
        mul_res.extend_from_slice(b);
        mul_res.extend_from_slice(i);

        let mul_ic = mul(&*mul_res)?;

        let mut acc_mul_ic = Vec::new();
        acc_mul_ic.extend_from_slice(&acc);
        acc_mul_ic.extend_from_slice(&mul_ic);

        acc = add(&*acc_mul_ic)?;
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
    pairing(&input[..])
}
