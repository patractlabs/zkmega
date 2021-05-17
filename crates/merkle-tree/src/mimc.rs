use alloc::vec::Vec;

use once_cell::sync::Lazy;
use tiny_keccak::{Hasher, Keccak};
use zkp_u256::{Zero, U256};

// Implements MiMC-p/p(it is possible to use MiMC-2n/n for large block size)
// over the AltBn128 scalar field used by zksnarks
// Paper: https://eprint.iacr.org/2016/492.pdf
// Round constants are generated in sequence from a seed

// Keccak IV seed.
const SEED: &str = "mimc";

static SCALAR_FIELD: Lazy<U256> = Lazy::new(|| {
    U256::from_decimal_str(
        "21888242871839275222246405745257275088548364400416034343698204186575808495617",
    )
    .unwrap()
});

/// MiMC-p/p with exponent of 7
/// Recommended at least 46 rounds, for a polynomial degree of 2^126
// basic compressed function(based on Even-Mansour mode
fn mimc_pe7(in_x: &U256, in_k: &U256, in_seed: &U256, round_count: u64) -> U256 {
    // Initialise round constants, k will be hashed
    if round_count < 1 {
        return U256::from(0);
    }
    let mut c = in_seed.clone();
    let mut t: U256;
    let mut a: U256;
    let mut in_x = in_x.clone();

    // Further n-2 subsequent rounds include a round constant
    for _ in 0..round_count {
        let mut keccak = Keccak::v256();
        let mut received = [0u8; 32];
        keccak.update(&c.to_bytes_be()[..]);
        keccak.finalize(&mut received);
        c = U256::from_bytes_be(&received) % &*SCALAR_FIELD;

        // x = (x + c_i + k)^7
        t = &in_x + &c % &*SCALAR_FIELD + in_k % &*SCALAR_FIELD; // t = x + c_i + k
        a = t.mulmod(&t, &*SCALAR_FIELD); // t^2
        a = a.mulmod(&a, &*SCALAR_FIELD).mulmod(&a, &*SCALAR_FIELD);
        in_x = a.mulmod(&t, &*SCALAR_FIELD); // t^7
    }

    // Return adds key again as blinding factor
    (in_x + in_k) % &*SCALAR_FIELD
}

// Sponge mode instantiated by MiMC permutation with a fixed key
fn mimc_pe7_mp(in_x: Vec<&U256>, in_k: &U256, in_seed: U256, round_count: u64) -> U256 {
    let mut r = in_k.clone();
    for i in in_x {
        r = &r + i + mimc_pe7(i, &r, &in_seed, round_count) % &*SCALAR_FIELD;
    }
    r
}

pub fn mimc_with_key(msg: Vec<&U256>, in_key: &U256) -> U256 {
    let mut keccak = Keccak::v256();
    let mut seed = [0u8; 32];
    keccak.update(SEED.as_ref());
    keccak.finalize(&mut seed);
    let in_seed = U256::from_bytes_be(&seed) % &*SCALAR_FIELD;
    mimc_pe7_mp(msg, in_key, in_seed, 91)
}

// padding message to 32 bytes size.
pub fn padding_message(msg: &[u8]) -> Vec<U256> {
    let quotient = msg.len() / 32;
    let mut padding_msg = Vec::new();
    let mut u256_array = [0u8; 32];
    (0..quotient).for_each(|i| {
        u256_array.copy_from_slice(&msg[i..i + 32]);
        padding_msg.push(U256::from_bytes_be(&u256_array))
    });

    if msg.len() % 32 != 0 {
        let mut padding_array = [0u8; 32];
        padding_array[32 - msg.len()..].copy_from_slice(&msg[quotient * 32..msg.len()]);
        let last_u256 = U256::from_bytes_be(&padding_array);
        padding_msg.push(last_u256);
    };
    padding_msg
}

pub fn mimc(msg: &[u8]) -> U256 {
    let in_k = U256::zero();
    let padding_msg = padding_message(msg);
    mimc_with_key(padding_msg.iter().collect(), &in_k)
}

#[test]
fn test_mimc() {
    let message = U256::from_decimal_str("49").unwrap();
    let in_key = U256::zero();

    assert_eq!(mimc(b"1"), mimc_with_key(vec![&message], &in_key));
}
