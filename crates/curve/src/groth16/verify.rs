//! Groth16 verifaction
use crate::{result::Result, CurveBasicOperations, Error, ErrorKind, SerializationError};
use alloc::vec::Vec;
use num_bigint::BigUint;
use parity_scale_codec::{Decode, Encode};

/// Groth16 Verifying Parcel
#[derive(Debug, Encode, Decode)]
pub struct Groth16Parcel {
    pub vk_gamma_abc: Vec<Vec<u8>>,
    pub vk: Vec<u8>,
    pub proof: Vec<u8>,
    pub public_inputs: Vec<Vec<u8>>,
}

/// Verify Wrapper
pub fn verify<C: CurveBasicOperations>(parcel: Vec<u8>) -> Result<bool> {
    let Groth16Parcel {
        vk_gamma_abc,
        vk,
        proof,
        public_inputs,
    } = Groth16Parcel::decode(&mut parcel.as_ref()).map_err(|_| Error::VerifyParcelFailed)?;
    verify_proof::<C>(vk_gamma_abc, vk, proof, public_inputs)
}

/// Groth16 verification
pub fn verify_proof<C: CurveBasicOperations>(
    vk_gamma_abc: Vec<Vec<u8>>,
    vk: Vec<u8>,
    proof: Vec<u8>,
    public_inputs: Vec<Vec<u8>>,
) -> Result<bool> {
    let g1_len = C::G1_LEN;
    let g2_len = C::G2_LEN;
    let g1_g2_len = C::G2_LEN + C::G1_LEN;
    let scalar_len = C::SCALAR_LEN;

    if (public_inputs.len() + 1) != vk_gamma_abc.len() {
        return Err(Error::VerifyParcelFailed);
    }

    // First two fields are used as the sum
    let mut acc = vk_gamma_abc[0].to_vec();

    // Compute the linear combination vk_x
    //  [(βui(x)+αvi(x)+wi(x))/γ] ∈ G1
    // acc = sigma(i:0~l)* [(βui(x)+αvi(x)+wi(x))/γ] ∈ G1
    for (i, b) in public_inputs.iter().zip(vk_gamma_abc.iter().skip(1)) {
        let mut mul_input = vec![0; g1_len];
        mul_input.copy_from_slice(b);
        mul_input.extend_from_slice(i);

        // Check if invalid length
        // if mul_input.len() != g1_len + scalar_len {
        //     return Err(format!(
        //         "Invalid input length {} for mul operation, should be {}",
        //         mul_input.len(),
        //         g1_len * 2
        //     )
        //     .into());
        // }
        let mul_ic = crate::call(0x01000001 + C::CURVE_ID, &mul_input)?;

        let mut acc_mul_ic = vec![0; g1_len];
        acc_mul_ic.copy_from_slice(acc.as_ref());
        acc_mul_ic.extend_from_slice(mul_ic.as_ref());

        // Check if invalid length
        // if acc_mul_ic.len() != g1_len * 2 {
        //     return Err(format!(
        //         "Invalid input length {} for add operation, should be {}",
        //         acc_mul_ic.len(),
        //         g1_len * 2
        //     )
        //     .into());
        // }
        acc = crate::call(0x01000000 + C::CURVE_ID, &*acc_mul_ic)?;
    }

    // The original verification equation is:
    // A * B = alpha * beta + acc * gamma + C * delta
    // ... however, we rearrange it so that it is:
    // A * B - acc * gamma - C * delta = alpha * beta
    // or equivalently:
    //    A   *    B    +  (-acc) * gamma +  (-C) * delta  +   (-alpha) * beta = 0
    let pairings = [
        (
            &proof[0..g1_len / 2],           // G1 x
            &proof[g1_len / 2..g1_len - 1],  // G1 y
            &proof[g1_len - 1..g1_len],      // G1 infinity
            &proof[g1_len..g1_len + g2_len], // G2
        ),
        (
            &acc[0..g1_len / 2],
            &*negate_y::<C>(&acc[g1_len / 2..g1_len - 1])?,
            &acc[g1_len - 1..g1_len],
            &vk[0..g2_len],
        ),
        (
            &proof[g1_g2_len..g1_g2_len + g1_len / 2],
            &*negate_y::<C>(&proof[g1_g2_len + g1_len / 2..g1_g2_len + g1_len - 1])?,
            &proof[g1_g2_len + g1_len - 1..g1_g2_len + g1_len],
            &vk[g2_len..g2_len * 2],
        ),
        (
            &vk[g2_len * 2..g2_len * 2 + g1_len / 2],
            &*negate_y::<C>(&vk[g2_len * 2 + g1_len / 2..g2_len * 2 + g1_len - 1])?,
            &vk[g2_len * 2 + g1_len - 1..g2_len * 2 + g1_len],
            &vk[g2_len * 2 + g1_len..g2_len * 3 + g1_len],
        ),
    ];

    let mut input = Vec::with_capacity((g1_len + g2_len) * 4);
    pairings.iter().for_each(|(x, y, infinity, g2)| {
        input.extend_from_slice(x);
        input.extend_from_slice(y);
        input.extend_from_slice(infinity);
        input.extend_from_slice(g2);
    });

    // Return the result of computing the pairing check
    // e(p1[0], p2[0]) *  .... * e(p1[n], p2[n]) == 1.
    // For example pairing([P1(), P1().negate()], [P2(), P2()]) should return true.
    Ok(crate::call(0x01000002 + C::CURVE_ID, &input)?[0] == 0)
}

fn negate_y_based_curve(y: BigUint, MODULUS: &[u8]) -> BigUint {
    let q = BigUint::from_bytes_le(MODULUS);
    q.clone() - y.clone() % q
}

fn negate_y<C: CurveBasicOperations>(y: &[u8]) -> Result<Vec<u8>> {
    let neg_y = negate_y_based_curve(BigUint::from_bytes_le(y), C::MODULUS).to_bytes_le();

    // Because of randomness, Negate_y vector might not satisfy g1_y_len bytes.
    let mut neg_y_fill_with_zero = vec![0; y.len()];
    neg_y_fill_with_zero[0..neg_y.len()].copy_from_slice(&*neg_y);

    Ok(neg_y_fill_with_zero)
}
