//! # Megaclite
//!
//! [Megaclite][book] is a zero-knowledge proof tool set building for the Polkadot ecology.
//!
//! + [Pairing](https://patractlabs.github.io/megaclite/pairing)
//! + [Pallet Contracts](https://patractlabs.github.io/megaclite/pallet-contracts)
//! + [Metis](https://patractlabs.github.io/megaclite/metis)
//!
//!
//! ## ZK Rollup Introduction
//!
//! Compared with the privacy function, the performance improvement brought by Rollup is the
//! early application direction of zero-knowledge proof. At present, the Layer 2 expansion
//! plan of the blockchain is to transfer a considerable part of the on-chain workload to
//! off-chain to complete, and the most watched one is ZK Rollup. The essence of ZK Rollup
//! is to compress the application on-chain state and store it in a Merkle tree, and move
//! the state transition funtions to off-chain. At the same time, the correctness of the
//! off-chain state transition process is guaranteed through the proof of zkSNARK. Compared
//! with the high cost of directly processing state changes on the chain, the ZK Proof's
//! on-chain smart contract verification is extremely cost low. At the same time, the
//! compressed information will also be submitted to the chain together with the proof,
//! which ensures data availability and obtains the same level of security as Layer 1.
//!
//! The Ethereum Layer 2 protocols related to ZK Rollup are: [zkSync][zkSync], [aztec][aztec],
//! etc. Their contract verification modules share a part of the elliptic curve's basic algorithms.
//! In 2017, Ethereum integrated three basic cryptographic calculation units of the alt
//! bn128 curve in the form of pre-compiled contracts, which are [EIP196][EIP196]’s ADD and Scalar_MUL
//! algorithms, and [EIP197][EIP197]’s Pairing algorithm. On top of this, due to the lack of rapid
//! upgrade capabilities of Ethereum, the community can only encapsulate some  tool libraries
//! through costly Solidity contracts. On top of these basic contract  libraries, many DApps can combine
//! ZK Rollup technology to achieve some innovations, such as [loopring][loopring], [gitcoin][gitcoin]
//! and [uniswap][uniswap] etc. However, in the past 3 years, ZK technology has further developed,
//! such as the more practical [BLS curve][BLS curve], and [PLONK algorithm][PLONK algorithm] etc.
//! Ethereum has not yet supported it.
//!
//!
//! ## LICENSE
//!
//! Apache-2.0
//!
//! [book]: https://patractlabs.github.io/megaclite
//! [zkSync]: https://zksync.io/
//! [aztec]: https://aztec.network/
//! [EIP196]: https://github.com/ethereum/EIPs/blob/master/EIPS/eip-196.md
//! [EIP197]: https://github.com/ethereum/EIPs/blob/master/EIPS/eip-197.md
//! [gitcoin]: https://gitcoin.co/
//! [uniswap]: https://uniswap.org/
//! [loopring]: https://loopring.org/
//! [BLS curve]: https://electriccoin.co/blog/new-snark-curve/
//! [PLONK algorithm]: https://eprint.iacr.org/2019/953/20190827:165656
mod scratch;

pub mod altbn_128;
pub mod bls12_381;
pub mod parse;
pub mod raw_bn_bls;
pub mod result;

use num_bigint::BigUint;
use num_traits::Num;
use result::{Error::Megaclite, Result};

static BN256_SCALAR_FIELD: &'static str =
    "21888242871839275222246405745257275088548364400416034343698204186575808495617";

static BN256_PRIME_FIELD: &'static str =
    "21888242871839275222246405745257275088696311157297823662689037894645226208583";

static BLS381_SCALAR_FIELD: &'static str =
    "52435875175126190479447740508185965837690552500527637822603658699938581184513";

static BLS381_PRIME_FIELD: &'static str =
    "4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787";

fn negate_y_based_curve(y: BigUint, prime_field: &'static str) -> Result<BigUint> {
    let q = BigUint::from_str_radix(prime_field, 10)?;
    let q_clone = q.clone();
    Ok(q - y % q_clone)
}

fn negate_y(y: &[u8]) -> Result<Vec<u8>> {
    let negate_y = BigUint::from_bytes_be(y);
    let neg_y = match y.len() {
        32 => negate_y_based_curve(negate_y, BN256_PRIME_FIELD)?.to_bytes_be(),
        48 => negate_y_based_curve(negate_y, BLS381_PRIME_FIELD)?.to_bytes_be(),
        _ => return Err(Megaclite("Invalid y coordinate length!".to_string())),
    };
    // Because of randomness, Negate_y vector might not satisfy 32 or 48 bytes.
    let mut neg_y_fill_with_zero = vec![0u8; y.len()];
    if neg_y.len() != y.len() {
        neg_y_fill_with_zero[y.len() - neg_y.len()..y.len()].copy_from_slice(&*neg_y);
    } else {
        neg_y_fill_with_zero[0..y.len()].copy_from_slice(&*neg_y);
    }
    Ok(neg_y_fill_with_zero)
}
