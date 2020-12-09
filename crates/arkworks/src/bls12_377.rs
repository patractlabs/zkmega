use super::CurveBasicOperations;
use ark_bls12_377::{Bls12_377, Fq12, Fr, G1Projective, G2Projective};
use ark_ec::PairingEngine;
use ark_ff::{
    fields::{Field, PrimeField},
    test_rng, One,
};
use ark_std::{ops::MulAssign, vec::Vec};
use rand::Rng;
use rustc_hex::FromHex;

impl CurveBasicOperations for Bls12_377 {
    const SCALAR_FIELD: &'static str = "";
    const PRIME_FIELD: &'static str = "258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458177";
    const G1_LEN: usize = 97;
    const G2_LEN: usize = 193;
    const SCALAR_LEN: usize = 32;
}

paste_test!();
