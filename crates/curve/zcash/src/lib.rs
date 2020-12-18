#![allow(unused_doc_comments)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(non_snake_case)]
pub mod alt_bn128;
pub mod bls12_381;
mod verifier;

use core::convert::TryFrom;
use num_bigint::BigUint;
use num_traits::Num;
pub use verifier::verify_proof;

pub trait Curve {
    // curve parameters
    const SCALAR_FIELD: &'static str;
    const MODULUS: &'static str;
    // Fq bytes length of different curves.
    const FQ_BYTES_LENGTH: usize;
    // Byte array of input elliptic curve points.
    type Point: AsRef<[u8]> + for<'a> TryFrom<&'a [u8]>;
    // Add two points on an elliptic curve.
    fn point_add(input: &[u8]) -> Result<Self::Point, &'static str>;

    // Scalar multiplication on elliptic curve.
    fn point_scalar_mul(input: &[u8]) -> Result<Self::Point, &'static str>;

    // Pairing n pairs of points on elliptic curve.
    fn point_pairing(input: &[u8]) -> Result<bool, &'static str>;
}
