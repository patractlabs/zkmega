use super::CurveBasicOperations;
use ark_bls12_381::{Bls12_381, Fq12, Fr, G1Projective, G2Projective};
use ark_ec::PairingEngine;
use ark_ff::{test_rng, Field, One, PrimeField};
use ark_std::{ops::MulAssign, vec::Vec};
use rand::Rng;
use rustc_hex::FromHex;

impl CurveBasicOperations for Bls12_381 {
    const SCALAR_FIELD: &'static str =
        "52435875175126190479447740508185965837690552500527637822603658699938581184513";
    const PRIME_FIELD: &'static str = "4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787";
    const G1_LEN: usize = 97;
    const G2_LEN: usize = 193;
    const SCALAR_LEN: usize = 32;
}

paste_test!();