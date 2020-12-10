#![allow(unused_doc_comments)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(non_snake_case)]
#![no_std]
mod derive;

pub mod curve;
pub mod groth16;
pub mod ops;
pub mod tests;

pub use ark_serialize::SerializationError;
pub use ark_std::io::{Error, ErrorKind};
use ark_std::{ops::MulAssign, vec::Vec};
pub use ops::CurveBasicOperations;

/// Vector Addition
#[cfg(not(feature = "ink"))]
pub fn add(curve_id: i32, input: &[u8]) -> Result<Vec<u8>, SerializationError> {
    match curve_id {
        0x2a => <ark_bls12_377::Bls12_377 as CurveBasicOperations>::add(input),
        0x2b => <ark_bls12_381::Bls12_381 as CurveBasicOperations>::add(input),
        0x2c => <ark_bn254::Bn254 as CurveBasicOperations>::add(input),
        0x2d => <ark_bw6_761::BW6_761 as CurveBasicOperations>::add(input),
        _ => Err(Error::new(ErrorKind::Other, "Invalid curve id").into()),
    }
}

/// Vector Addition
#[cfg(feature = "ink")]
pub fn add(curve_id: i32, input: &[u8]) -> Result<bool, SerializationError> {
    ink_env::pairing::add(curve_id, input)
}

/// Scalar MulAssign
#[cfg(not(feature = "ink"))]
pub fn mul(curve_id: i32, input: &[u8]) -> Result<Vec<u8>, SerializationError> {
    match curve_id {
        0x2a => <ark_bls12_377::Bls12_377 as CurveBasicOperations>::mul(input),
        0x2b => <ark_bls12_381::Bls12_381 as CurveBasicOperations>::mul(input),
        0x2c => <ark_bn254::Bn254 as CurveBasicOperations>::mul(input),
        0x2d => <ark_bw6_761::BW6_761 as CurveBasicOperations>::mul(input),
        _ => Err(Error::new(ErrorKind::Other, "Invalid curve id").into()),
    }
}

/// Scalar MulAssign
#[cfg(feature = "ink")]
pub fn mul(curve_id: i32, input: &[u8]) -> Result<bool, SerializationError> {
    ink_env::pairing::add(curve_id, input)
}

/// Pairing
#[cfg(not(feature = "ink"))]
pub fn pairing(curve_id: i32, input: &[u8]) -> Result<bool, SerializationError> {
    match curve_id {
        0x2a => <ark_bls12_377::Bls12_377 as CurveBasicOperations>::pairings(input),
        0x2b => <ark_bls12_381::Bls12_381 as CurveBasicOperations>::pairings(input),
        0x2c => <ark_bn254::Bn254 as CurveBasicOperations>::pairings(input),
        0x2d => <ark_bw6_761::BW6_761 as CurveBasicOperations>::pairings(input),
        _ => Err(Error::new(ErrorKind::Other, "Invalid curve id").into()),
    }
}

/// Pairing
#[cfg(feature = "ink")]
pub fn pairing(curve_id: i32, input: &[u8]) -> Result<bool, SerializationError> {
    ink_env::pairing::add(curve_id, input)
}

/// Groth16 Verifaction
#[cfg(not(feature = "ink"))]
pub fn groth16_verify(
    curve_id: i32,
    vk_gamma_abc: &[&[u8]],
    vk: &[u8],
    proof: &[u8],
    public_inputs: &[&[u8]],
) -> Result<bool, SerializationError> {
    match curve_id {
        0x2a => groth16::verify::<ark_bls12_377::Bls12_377>(vk_gamma_abc, vk, proof, public_inputs),
        0x2b => groth16::verify::<ark_bls12_381::Bls12_381>(vk_gamma_abc, vk, proof, public_inputs),
        0x2c => groth16::verify::<ark_bn254::Bn254>(vk_gamma_abc, vk, proof, public_inputs),
        0x2d => groth16::verify::<ark_bw6_761::BW6_761>(vk_gamma_abc, vk, proof, public_inputs),
        _ => Err(Error::new(ErrorKind::Other, "Invalid curve id").into()),
    }
}
