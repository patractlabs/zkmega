#![allow(unused_doc_comments)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(non_snake_case)]
#![no_std]
#[macro_use]
extern crate alloc;

mod derive;

pub mod curve;
pub mod groth16;
pub mod ops;
pub mod result;
pub mod tests;

pub use ark_serialize::SerializationError;
pub use ark_std::io::{Error, ErrorKind};
use ark_std::{ops::MulAssign, vec::Vec};
pub use ops::CurveBasicOperations;
use result::Result;

/// bool to bytes
#[cfg(not(feature = "ink"))]
fn b2b(b: bool) -> Vec<u8> {
    Vec::from(if b { [0] } else { [1] })
}

/// Call curve function
#[cfg(feature = "ink")]
pub fn call(func_id: u32, input: &[u8]) -> Result<Vec<u8>> {
    Ok(ink_env::call_chain_extension(func_id, &Vec::from(input))?)
}

/// Call curve function
#[cfg(not(feature = "ink"))]
pub fn call(func_id: u32, input: &[u8]) -> Result<Vec<u8>> {
    Ok(match func_id {
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
        _ => Err(Error::new(ErrorKind::Other, "Invalid function id").into()),
    }?)
}
