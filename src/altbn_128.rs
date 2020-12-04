use crate::*;
use crate::{
    result::Result,
    scratch::{Bytes, Curve},
};
use bellman_ce::pairing::bn256::Bn256;
use core::convert::TryInto;
use num_bigint::BigUint;
use num_traits::Num;

curve! {Bn256, 64, 128}
