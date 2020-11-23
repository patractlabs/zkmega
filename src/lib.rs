extern crate alloc;

pub mod altbn_128;
pub mod bls12_381;
pub mod parse;
mod scratch;

// use alloc::vec::Vec;
// use bls12_381::{bls381_add, bls381_pairing, bls381_scalar_mul};
// use core::convert::TryInto;
// use num_bigint::BigInt;
// use num_traits::Num;
// use zkp_u256::U256;

// static SCALAR_FIELD: &'static str =
//     "21888242871839275222246405745257275088548364400416034343698204186575808495617";
//
// /// docs: TODO
// pub fn negate_y_u256(y: U256) -> U256 {
//     let q = U256::from_decimal_str(
//         "21888242871839275222246405745257275088696311157297823662689037894645226208583",
//     )
//     .expect("Wrong U256");
//     let q_clone = q.clone();
//     q - y % q_clone
// }
//
// /// docs: TODO
// pub fn negate(y: BigInt) -> BigInt {
//     let q = BigInt::from_str_radix(
//         "21888242871839275222246405745257275088696311157297823662689037894645226208583",
//         10,
//     )
//     .expect("Wrong BigInt");
//     let q_clone = q.clone();
//     q - y % q_clone
// }
//
// /// docs: TODO
// fn negate_y_slice(y: &[u8]) -> Vec<u8> {
//     let negate_y = BigInt::from_signed_bytes_be(y);
//     negate(negate_y).to_signed_bytes_be()
// }
//
// /// docs: TODO
// pub fn verify_proof(
//     vk_gamma_abc: &[&[u8]],
//     vk: &[u8],
//     proof: &[u8],
//     public_inputs: &[&[u8]],
// ) -> Result<bool, &'static str> {
//     if (public_inputs.len() + 1) != vk_gamma_abc.len() {
//         return Err("verifying key was malformed.");
//     }
//
//     // First two fields are used as the sum
//     let mut acc: [u8; 96] = if vk_gamma_abc[0].len() != 96 {
//         return Err("vk_gamma_abc first element length isn't 96,Invalid length!");
//     } else {
//         vk_gamma_abc[0].try_into().unwrap()
//     };
//
//     // Compute the linear combination vk_x
//     //  [(βui(x)+αvi(x)+wi(x))/γ] ∈ G1
//     // acc = sigma(i:0~l)* [(βui(x)+αvi(x)+wi(x))/γ] ∈ G1
//     for (i, b) in public_inputs.iter().zip(vk_gamma_abc.iter().skip(1)) {
//         if BigInt::from_signed_bytes_be(i)
//             < BigInt::from_str_radix(SCALAR_FIELD, 10).expect("wrong ")
//         {
//             return Err("Invalid public input!");
//         }
//         let mut mul = Vec::new();
//         mul.extend_from_slice(b);
//         mul.extend_from_slice(i);
//
//         let mul_ic = bls381_scalar_mul(&*mul)?;
//
//         let mut acc_mul_ic = Vec::new();
//         acc_mul_ic.extend_from_slice(&acc);
//         acc_mul_ic.extend_from_slice(&mul_ic);
//
//         acc = bls381_add(&*acc_mul_ic)?;
//     }
//
//     // The original verification equation is:
//     // A * B = alpha * beta + acc * gamma + C * delta
//     // ... however, we rearrange it so that it is:
//     // A * B - acc * gamma - C * delta = alpha * beta
//     // or equivalently:
//     // A * B + (-acc) * gamma + (-C) * delta + (-alpha) * beta = 0
//     // which allows us to do a single final exponentiation.
//     let mut input = Vec::new();
//     input.extend_from_slice(&proof[0..96]); // A
//     input.extend_from_slice(&proof[96..96 * 3]); // B
//
//     let negate_acc = negate_y_slice(&acc);
//     input.extend_from_slice(&*negate_acc); // -acc
//     input.extend_from_slice(&vk[0..96 * 2]); // γ
//
//     input.extend_from_slice(&proof[96 * 3..96 * 3 + 48]);
//     let negate_c = negate_y_slice(&proof[96 * 3 + 48..96 * 4]);
//     input.extend_from_slice(&*negate_c); // -C
//     input.extend_from_slice(&vk[96 * 2..96 * 4]); // δ
//
//     input.extend_from_slice(&vk[96 * 4..96 * 4 + 48]);
//     let negate_gamma = negate_y_slice(&vk[96 * 4 + 48..96 * 5]);
//     input.extend_from_slice(&*negate_gamma); // -α
//     input.extend_from_slice(&vk[96 * 5..96 * 7]); // β
//
//     // Return the result of computing the pairing check
//     // e(p1[0], p2[0]) *  .... * e(p1[n], p2[n]) == 1.
//     // For example pairing([P1(), P1().negate()], [P2(), P2()]) should return true.
//     bls381_pairing(&input[..])
// }
//
// #[test]
// fn test_verify_proof() {}
