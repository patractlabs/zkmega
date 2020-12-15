use super::*;
use alloc::vec::Vec;
use core::mem;
use once_cell::sync::Lazy;
use tiny_keccak::{Hasher, Sha3};
use zkp_u256::U256;

static GENERATE: Lazy<Vec<U256>> = Lazy::new(|| {
    let generator = [
        "17777552123799933955779906779655732241715742912184938656739573121738514868268",
        "2626589144620713026669568689430873010625803728049924121243784502389097019475",
    ];
    generator
        .iter()
        .map(|g| U256::from_decimal_str(g).unwrap())
        .collect::<Vec<U256>>()
});

pub fn hash_to_u256(data: &[u8]) -> U256 {
    let mut hashed = [0u8; 32];
    let mut sha256 = Sha3::v256();
    sha256.update(data);
    sha256.finalize(&mut hashed);

    let hash_u256 = U256::from_bytes_be(&hashed);
    // (2<<249) - 1
    let mask = U256::from_decimal_str(
        "1809251394333065553493296640760748560207343510400633813116524750123642650623",
    )
    .unwrap();

    // hash mod mask.
    return hash_u256 & mask;
}

pub fn verify(public_key: [U256; 2], r: [U256; 2], hashed_msg: U256, s: U256) -> bool {
    let mut input = Vec::with_capacity(32 * 5);
    input.extend_from_slice(unsafe { &mem::transmute_copy::<[U256; 2], [u8; 32]>(&r) });
    input.extend_from_slice(unsafe { &mem::transmute_copy::<[U256; 2], [u8; 32]>(&public_key) });
    input.extend_from_slice(&hashed_msg.to_bytes_be());

    if let Some(lhs) = scalar_mult(GENERATE[0].clone(), GENERATE[1].clone(), s) {
        let t = hash_to_u256(&input);
        if let Some(rhs) = scalar_mult(public_key[0].clone(), public_key[1].clone(), t) {
            return lhs == rhs;
        }
    }
    false
}
