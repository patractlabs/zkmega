//! Groth16 Verify
use arkworks_curve::{CurveBasicOperations, Error, ErrorKind, SerializationError};
use num_bigint::BigUint;
use num_traits::Num;

/// Groth16 verification
pub fn verify<C: CurveBasicOperations>(
    vk_gamma_abc: &[&[u8]],
    vk: &[u8],
    proof: &[u8],
    public_inputs: &[&[u8]],
) -> Result<bool, SerializationError> {
    let g1_len = C::G1_LEN;
    let g2_len = C::G2_LEN;
    let g1_g2_len = C::G2_LEN + C::G1_LEN;
    let scalar_len = C::SCALAR_LEN;

    if (public_inputs.len() + 1) != vk_gamma_abc.len() {
        return Err(Error::new(ErrorKind::Other, "Verifying key was malformed"))?;
    }

    // First two fields are used as the sum
    let mut acc = vk_gamma_abc[0].to_vec();

    // Compute the linear combination vk_x
    //  [(βui(x)+αvi(x)+wi(x))/γ] ∈ G1
    // acc = sigma(i:0~l)* [(βui(x)+αvi(x)+wi(x))/γ] ∈ G1
    for (i, b) in public_inputs.iter().zip(vk_gamma_abc.iter().skip(1)) {
        public_input_require_on_curve::<C>(i).map_err(|e| Error::new(ErrorKind::Other, e))?;

        let mut mul_input = vec![0u8; g1_len + scalar_len];
        mul_input[0..g1_len].copy_from_slice(b);
        mul_input[g1_len..g1_len + scalar_len].copy_from_slice(i);

        let mul_ic = C::mul(&mul_input)?;

        let mut acc_mul_ic = vec![0u8; g1_len * 2];
        acc_mul_ic[0..g1_len].copy_from_slice(acc.as_ref());
        acc_mul_ic[g1_len..g1_len * 2].copy_from_slice(mul_ic.as_ref());

        acc = C::add(&*acc_mul_ic)?;
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
            &*negate_y::<C>(&acc[g1_len / 2..g1_len - 1])
                .map_err(|e| Error::new(ErrorKind::Other, e))?,
            &acc[g1_len - 1..g1_len],
            &vk[0..g2_len],
        ),
        (
            &proof[g1_g2_len..g1_g2_len + g1_len / 2],
            &*negate_y::<C>(&proof[g1_g2_len + g1_len / 2..g1_g2_len + g1_len - 1])
                .map_err(|e| Error::new(ErrorKind::Other, e))?,
            &proof[g1_g2_len + g1_len - 1..g1_g2_len + g1_len],
            &vk[g2_len..g2_len * 2],
        ),
        (
            &vk[g2_len * 2..g2_len * 2 + g1_len / 2],
            &*negate_y::<C>(&vk[g2_len * 2 + g1_len / 2..g2_len * 2 + g1_len - 1])
                .map_err(|e| Error::new(ErrorKind::Other, e))?,
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
    C::pairings(&input[..])
}

fn negate_y_based_curve(y: BigUint, prime_field: &'static str) -> Result<BigUint, &'static str> {
    let q = BigUint::from_str_radix(prime_field, 10)
        .map_err(|_| "Wrong curve parameter:PRIME_FIELD")?;
    let q_clone = q.clone();
    Ok(q - y % q_clone)
}

fn negate_y<C: CurveBasicOperations>(y: &[u8]) -> Result<Vec<u8>, &'static str> {
    let neg_y = negate_y_based_curve(BigUint::from_bytes_be(y), C::PRIME_FIELD)?.to_bytes_be();

    // Because of randomness, Negate_y vector might not satisfy 32 or 48 bytes.
    let mut neg_y_fill_with_zero = vec![0u8; y.len()];
    if neg_y.len() != y.len() {
        neg_y_fill_with_zero[y.len() - neg_y.len()..y.len()].copy_from_slice(&*neg_y);
    } else {
        neg_y_fill_with_zero[0..y.len()].copy_from_slice(&*neg_y);
    }
    Ok(neg_y_fill_with_zero)
}

fn public_input_require_on_curve<C: CurveBasicOperations>(
    input: &[u8],
) -> Result<(), &'static str> {
    if BigUint::from_bytes_be(input)
        >= BigUint::from_str_radix(C::SCALAR_FIELD, 10)
            .map_err(|_| "Parse wrong: public input to BigUint.")?
    {
        return Err("public input is invalid.");
    }
    Ok(())
}
