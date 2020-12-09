use super::CurveBasicOperations;
use ark_bn254::{Bn254, Fq12, Fr, G1Projective, G2Projective};
use ark_ec::PairingEngine;
use ark_ff::{test_rng, Field, One, PrimeField};
use ark_std::{ops::MulAssign, vec::Vec};
use rand::Rng;
use rustc_hex::FromHex;

impl CurveBasicOperations for Bn254 {
    const SCALAR_FIELD: &'static str =
        "21888242871839275222246405745257275088548364400416034343698204186575808495617";
    const PRIME_FIELD: &'static str =
        "21888242871839275222246405745257275088696311157297823662689037894645226208583";
    const G1_LEN: usize = 65;
    const G2_LEN: usize = 129;
    const SCALAR_LEN: usize = 32;
}

paste_test!();
