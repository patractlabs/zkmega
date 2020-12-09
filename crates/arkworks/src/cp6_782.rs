use super::CurveBasicOperations;
use ark_cp6_782::{Fq6, Fr, G1Projective, G2Projective, CP6_782};
use ark_ec::PairingEngine;
use ark_ff::{test_rng, Field, One, PrimeField};
use ark_std::{ops::MulAssign, vec::Vec};
use rand::Rng;
use rustc_hex::FromHex;

impl CurveBasicOperations for CP6_782 {
    const SCALAR_FIELD: &'static str = "";
    const PRIME_FIELD: &'static str = "";
    const G1_LEN: usize = 209;
    const G2_LEN: usize = 625;
    const SCALAR_LEN: usize = 48;
}

paste_test!();
