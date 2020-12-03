//! Matter Labs Curves
#![deny(missing_docs)]
pub mod curve;
pub mod result;

pub use crate::{
    curve::{Bytes, Curve},
    result::Result,
};
pub use bellman_ce::pairing::{bls12_381::Bls12, bn256::Bn256};

/// Declare curve
#[macro_export]
macro_rules! curve {
    ($(($curve:ident, $g1:expr)),*) => {
        $(curve!($curve, $g1);)+
    };
    ($curve:ident, $g1:expr) => {
        /// Op Bytes
        impl Bytes for [u8; $g1] {
            fn default() -> [u8; $g1] {
                [0; $g1]
            }
        }

        impl Curve for $curve {
            type Output = [u8; $g1];
        }
    };
}

curve! {
    (Bn256, 64),
    (Bls12, 96)
}

/// Vector Addition
pub fn add<T: Curve>(input: &[u8]) -> Result<T::Output> {
    <T as Curve>::add(input)
}

/// Scalar MulAssign
pub fn mul<T: Curve>(input: &[u8]) -> Result<T::Output> {
    <T as Curve>::mul(input)
}

/// Pairing
pub fn pairing<T: Curve>(input: &[u8]) -> Result<bool> {
    <T as Curve>::pairing(input)
}
