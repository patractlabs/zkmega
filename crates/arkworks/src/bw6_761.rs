use super::CurveBasicOperations;
use ark_bw6_761::{Fq6, Fr, G1Projective, G2Projective, BW6_761};
use ark_ec::PairingEngine;
use ark_ff::{test_rng, Field, One, PrimeField};
use ark_std::{ops::MulAssign, vec::Vec};
use rand::Rng;
use rustc_hex::FromHex;

impl CurveBasicOperations for BW6_761 {
    const SCALAR_FIELD: &'static str = "";
    const PRIME_FIELD: &'static str = "";
    const G1_LEN: usize = 193;
    const G2_LEN: usize = 193;
    const SCALAR_LEN: usize = 48;
}

paste_test!();
