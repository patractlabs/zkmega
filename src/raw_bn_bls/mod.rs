pub mod alt_bn128;
pub mod bls12_381;

use num_bigint::BigUint;
use num_traits::Num;
use std::convert::TryFrom;

static BN256_SCALAR_FIELD: &'static str =
    "21888242871839275222246405745257275088548364400416034343698204186575808495617";
static BN256_PRIME_FIELD: &'static str =
    "21888242871839275222246405745257275088696311157297823662689037894645226208583";

static BLS381_SCALAR_FIELD: &'static str =
    "52435875175126190479447740508185965837690552500527637822603658699938581184513";
static BLS381_PRIME_FIELD: &'static str =
    "4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787";

pub trait Curve<'a> {
    // Byte array of input elliptic curve points.
    type Point: AsRef<[u8]> + TryFrom<&'a [u8]>;

    // Fq bytes length of different curves.
    fn fq_bytes_length() -> usize;

    // Add two points on an elliptic curve.
    fn point_add(input: &[u8]) -> Result<Self::Point, &'static str>;

    // Scalar multiplication on elliptic curve.
    fn point_scalar_mul(input: &[u8]) -> Result<Self::Point, &'static str>;

    // Pairing n pairs of points on elliptic curve.
    fn point_pairing(input: &[u8]) -> Result<bool, &'static str>;
}

pub fn verify_proof<'a, C: Curve<'a>>(
    vk_gammaABC: &[&'a [u8]],
    vk: &[u8],
    proof: &[u8],
    public_inputs: &[&[u8]],
) -> Result<bool, &'static str> {
    let len = C::fq_bytes_length();
    if (public_inputs.len() + 1) != vk_gammaABC.len() {
        return Err("verifying key was malformed.");
    }

    // First two fields are used as the sum
    let mut acc =
        C::Point::try_from(vk_gammaABC[0]).map_err(|_| "vk_gammaABC slice try_from fail")?;

    // Compute the linear combination vk_x
    //  [(βui(x)+αvi(x)+wi(x))/γ] ∈ G1
    // acc = sigma(i:0~l)* [(βui(x)+αvi(x)+wi(x))/γ] ∈ G1
    for (i, b) in public_inputs.iter().zip(vk_gammaABC.iter().skip(1)) {
        input_require_on_curve::<C>(i)?;
        let mut mul = Vec::new();
        mul.extend_from_slice(b);
        mul.extend_from_slice(i);

        let mul_ic = C::point_scalar_mul(&*mul)?;

        let mut acc_mul_ic = Vec::new();
        acc_mul_ic.extend_from_slice(acc.as_ref());
        acc_mul_ic.extend_from_slice(mul_ic.as_ref());

        acc = C::point_add(&*acc_mul_ic)?;
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
            &proof[0..len],
            &proof[len..len * 2],
            &proof[len * 2..len * 6],
        ),
        (
            &acc.as_ref()[0..len],
            &*negate_y(&acc.as_ref()[len..len * 2])?,
            &vk[0..len * 4],
        ),
        (
            &proof[len * 6..len * 7],
            &*negate_y(&proof[len * 7..len * 8])?,
            &vk[len * 4..len * 8],
        ),
        (
            &vk[len * 8..len * 9],
            &*negate_y(&vk[len * 9..len * 10])?,
            &vk[len * 10..len * 14],
        ),
    ];

    let mut input = vec![0u8; len * 6 * 4];
    pairings.iter().enumerate().for_each(|(i, (x, y, g2))| {
        input[6 * i * len..(6 * i + 1) * len].copy_from_slice(x);
        input[(6 * i + 1) * len..(6 * i + 2) * len].copy_from_slice(y);
        input[(6 * i + 2) * len..(6 * i + 6) * len].copy_from_slice(g2);
    });

    // Return the result of computing the pairing check
    // e(p1[0], p2[0]) *  .... * e(p1[n], p2[n]) == 1.
    // For example pairing([P1(), P1().negate()], [P2(), P2()]) should return true.
    C::point_pairing(&input[..])
}

fn negate_y_based_curve(y: BigUint, prime_field: &'static str) -> BigUint {
    let q = BigUint::from_str_radix(prime_field, 10).expect("Wrong BigUint");
    let q_clone = q.clone();
    q - y % q_clone
}

fn negate_y(y: &[u8]) -> Result<Vec<u8>, &'static str> {
    let negate_y = BigUint::from_bytes_be(y);
    let neg_y = match y.len() {
        32 => negate_y_based_curve(negate_y, BN256_PRIME_FIELD).to_bytes_be(),
        48 => negate_y_based_curve(negate_y, BLS381_PRIME_FIELD).to_bytes_be(),
        _ => return Err("Invalid y coordinate length!"),
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

fn input_require_on_curve<'a, E: Curve<'a>>(input: &[u8]) -> Result<(), &'static str> {
    match E::fq_bytes_length() {
        32 => {
            if BigUint::from_bytes_be(input)
                >= BigUint::from_str_radix(BN256_SCALAR_FIELD, 10)
                    .map_err(|_| "Parse BigUint wrong ")?
            {
                return Err("Invalid public input!");
            }
        }
        48 => {
            if BigUint::from_bytes_le(input)
                >= BigUint::from_str_radix(BLS381_SCALAR_FIELD, 10)
                    .map_err(|_| "Parse BigUint wrong ")?
            {
                return Err("Invalid public input!");
            }
        }
        _ => return Err(
            "The length of fq does not exist, perhaps here you need to add your own curve require",
        ),
    }
    Ok(())
}
