//! Tests as exports

mod bls12_377;
mod bls12_381;
mod bn254;
mod bw6_761;
mod cp6_782;

pub use self::{bls12_377::*, bls12_381::*, bn254::*, bw6_761::*, cp6_782::*};
