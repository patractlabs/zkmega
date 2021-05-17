#![allow(unused_doc_comments)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(non_snake_case)]
#![cfg_attr(not(features = "std"), no_std)]

#[macro_use]
extern crate alloc;

pub mod curve;
mod derive;
pub mod error;
pub mod groth16;
pub mod ops;
pub mod tests;

pub use ark_serialize::SerializationError;
pub use ark_std::{io::ErrorKind, ops::MulAssign, vec::Vec};

pub use self::error::{Error, Result};
pub use self::ops::CurveBasicOperations;

/// Call curve function
#[cfg(feature = "ink")]
pub fn call(func_id: u32, input: &[u8]) -> Result<Vec<u8>> {
    use ink_env::chain_extension::{ChainExtensionMethod, FromStatusCode};
    Ok(ChainExtensionMethod::build(func_id)
        .input::<&[u8]>()
        .output_result::<Vec<u8>, Error>()
        .ignore_error_code()
        .call(&input)?)
}

/// bool to bytes
#[cfg(not(feature = "ink"))]
fn b2b(b: bool) -> Vec<u8> {
    Vec::from(if b { [0] } else { [1] })
}

/// Call curve function
#[cfg(not(feature = "ink"))]
pub fn call(func_id: u32, input: &[u8]) -> Result<Vec<u8>> {
    Ok(match func_id {
        // debug
        0x2a => Ok(input.to_vec()),
        // add - 0x2a + 0,1,2,3
        0x01000000 => <ark_bls12_377::Bls12_377 as CurveBasicOperations>::add(input),
        0x01000010 => <ark_bls12_381::Bls12_381 as CurveBasicOperations>::add(input),
        0x01000020 => <ark_bn254::Bn254 as CurveBasicOperations>::add(input),
        0x01000030 => <ark_bw6_761::BW6_761 as CurveBasicOperations>::add(input),
        // mul - 0x3a + 0,1,2,3
        0x01000001 => <ark_bls12_377::Bls12_377 as CurveBasicOperations>::mul(input),
        0x01000011 => <ark_bls12_381::Bls12_381 as CurveBasicOperations>::mul(input),
        0x01000021 => <ark_bn254::Bn254 as CurveBasicOperations>::mul(input),
        0x01000031 => <ark_bw6_761::BW6_761 as CurveBasicOperations>::mul(input),
        // pairing - 0x4a + 0,1,2,3
        0x01000002 => <ark_bls12_377::Bls12_377 as CurveBasicOperations>::pairings(input).map(b2b),
        0x01000012 => <ark_bls12_381::Bls12_381 as CurveBasicOperations>::pairings(input).map(b2b),
        0x01000022 => <ark_bn254::Bn254 as CurveBasicOperations>::pairings(input).map(b2b),
        0x01000032 => <ark_bw6_761::BW6_761 as CurveBasicOperations>::pairings(input).map(b2b),
        id => return Err(Error::InvalidFunctionId),
    }?)
}

/// Integrate add
pub fn add(func_id: u32, input: &[u8]) -> Result<Vec<u8>> {
    Ok(match func_id {
        0x00 => call(0x01000000, input)?,
        0x10 => call(0x01000010, input)?,
        0x20 => call(0x01000020, input)?,
        0x30 => call(0x01000030, input)?,
        _ => return Err(Error::InvalidFunctionId),
    })
}

/// Scalar mul
pub fn mul(func_id: u32, input: &[u8]) -> Result<Vec<u8>> {
    Ok(match func_id {
        0x00 => call(0x01000001, input)?,
        0x10 => call(0x01000011, input)?,
        0x20 => call(0x01000021, input)?,
        0x30 => call(0x01000031, input)?,
        _ => return Err(Error::InvalidFunctionId),
    })
}

/// pairing
pub fn pairing(func_id: u32, input: &[u8]) -> Result<Vec<u8>> {
    Ok(match func_id {
        0x00 => call(0x01000002, input)?,
        0x10 => call(0x01000012, input)?,
        0x20 => call(0x01000022, input)?,
        0x30 => call(0x01000032, input)?,
        _ => return Err(Error::InvalidFunctionId),
    })
}

/// Groth16 Verify
pub fn verify(
    curve_id: u32,
    vk_gamma_abc: Vec<Vec<u8>>,
    vk: Vec<u8>,
    proof: Vec<u8>,
    public_inputs: Vec<Vec<u8>>,
) -> Result<bool> {
    match curve_id {
        0x00 => groth16::verify_proof::<curve::Bls12_377>(vk_gamma_abc, vk, proof, public_inputs),
        0x10 => groth16::verify_proof::<curve::Bls12_381>(vk_gamma_abc, vk, proof, public_inputs),
        0x20 => groth16::verify_proof::<curve::Bn254>(vk_gamma_abc, vk, proof, public_inputs),
        0x30 => groth16::verify_proof::<curve::BW6_761>(vk_gamma_abc, vk, proof, public_inputs),
        id => Err(Error::InvalidFunctionId),
    }
}

/// Groth16 Verify Parcel
pub fn verify_parcel(curve_id: u32, parcel: Vec<u8>) -> Result<bool> {
    match curve_id {
        0x00 => groth16::verify::<curve::Bls12_377>(parcel),
        0x10 => groth16::verify::<curve::Bls12_381>(parcel),
        0x20 => groth16::verify::<curve::Bn254>(parcel),
        0x30 => groth16::verify::<curve::BW6_761>(parcel),
        id => Err(Error::InvalidFunctionId),
    }
}
