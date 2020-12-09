#![allow(unused_doc_comments)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(non_snake_case)]
#![no_std]
mod derive;

mod bls12_377;
mod bls12_381;
mod bn254;
mod bw6_761;
mod cp6_782;
pub mod ops;
pub mod tests;

pub use ark_serialize::SerializationError;
pub use ark_std::io::{Error, ErrorKind};
use ark_std::{ops::MulAssign, vec::Vec};
pub use ops::CurveBasicOperations;

/// Vector Addition
pub fn add(curve_id: i32, input: &[u8]) -> Result<Vec<u8>, SerializationError> {
    match curve_id {
        0x2a => <ark_bls12_377::Bls12_377 as CurveBasicOperations>::add(input),
        0x2b => <ark_bls12_381::Bls12_381 as CurveBasicOperations>::add(input),
        0x2c => <ark_bn254::Bn254 as CurveBasicOperations>::add(input),
        0x2d => <ark_cp6_782::CP6_782 as CurveBasicOperations>::add(input),
        _ => Err(Error::new(ErrorKind::Other, "Invalid curve id"))?,
    }
}

/// Scalar MulAssign
pub fn mul(curve_id: i32, input: &[u8]) -> Result<Vec<u8>, SerializationError> {
    match curve_id {
        0x2a => <ark_bls12_377::Bls12_377 as CurveBasicOperations>::mul(input),
        0x2b => <ark_bls12_381::Bls12_381 as CurveBasicOperations>::mul(input),
        0x2c => <ark_bn254::Bn254 as CurveBasicOperations>::mul(input),
        0x2d => <ark_cp6_782::CP6_782 as CurveBasicOperations>::mul(input),
        _ => Err(Error::new(ErrorKind::Other, "Invalid curve id"))?,
    }
}

/// Pairing
pub fn pairing(curve_id: i32, input: &[u8]) -> Result<Vec<u8>, SerializationError> {
    match curve_id {
        0x2a => <ark_bls12_377::Bls12_377 as CurveBasicOperations>::mul(input),
        0x2b => <ark_bls12_381::Bls12_381 as CurveBasicOperations>::mul(input),
        0x2c => <ark_bn254::Bn254 as CurveBasicOperations>::mul(input),
        0x2d => <ark_cp6_782::CP6_782 as CurveBasicOperations>::mul(input),
        _ => Err(Error::new(ErrorKind::Other, "Invalid curve id"))?,
    }
}

/// Re-export curves
pub mod curve {
    pub use ark_bls12_377::Bls12_377;
    pub use ark_bls12_381::Bls12_381;
    pub use ark_bn254::Bn254;
    pub use ark_bw6_761::BW6_761;
    pub use ark_cp6_782::CP6_782;
}
