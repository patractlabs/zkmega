use crate::{
    result::Result,
    scratch::{Bytes, Curve},
};
use bellman_ce::pairing::bls12_381::Bls12;

curve! {Bls12, 96, 192}
