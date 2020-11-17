mod alt_bn128;
mod bls12_381;

pub use alt_bn128::{alt_bn128_add,alt_bn128_scalar_mul,alt_bn128_pairing};
pub use bls12_381::{bls381_add,bls381_scalar_mul, bls381_pairing};