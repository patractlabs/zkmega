#![cfg(any(test, feature = "tests"))]
mod curve;
mod mimc;

pub fn add(curve_id: i32) {
    match curve_id {
        0x2a => curve::bls12_377_add(),
        0x2b => curve::bls12_381_add(),
        0x2c => curve::bn254_add(),
        0x2d => curve::bw6_761_add(),
        _ => {}
    }
}

pub fn mul(curve_id: i32) {
    match curve_id {
        0x2a => curve::bls12_377_mul(),
        0x2b => curve::bls12_381_mul(),
        0x2c => curve::bn254_mul(),
        0x2d => curve::bw6_761_mul(),
        _ => {}
    }
}

pub fn pairing(curve_id: i32) {
    match curve_id {
        0x2a => curve::bls12_377_pairing(),
        0x2b => curve::bls12_381_pairing(),
        0x2c => curve::bn254_pairing(),
        0x2d => curve::bw6_761_pairing(),
        _ => {}
    }
}

pub fn pairing_six(curve_id: i32) {
    match curve_id {
        0x2a => curve::bls12_377_pairing_six(),
        0x2b => curve::bls12_381_pairing_six(),
        0x2c => curve::bn254_pairing_six(),
        0x2d => curve::bw6_761_pairing_six(),
        _ => {}
    }
}

pub fn verify(curve_id: i32) {
    match curve_id {
        0x2a => curve::bls12_377_verify(),
        0x2b => curve::bls12_381_verify(),
        0x2c => curve::bn254_verify(),
        0x2d => curve::bw6_761_verify(),
        _ => {}
    }
}
