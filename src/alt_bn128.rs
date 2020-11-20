use crate::scratch::Curve;
use pairing_ce::{bn256::Bn256, GroupDecodingError};

/// The G1 length of altbn_128
const G1_LENGTH: usize = 96;

/// altbn_128 add
pub fn add(input: &[u8], output: &mut [u8]) -> Result<(), GroupDecodingError> {
    <Bn256 as Curve>::add(input, output)
}

/// altbn_128 mul
pub fn mul(input: &[u8], output: &mut [u8]) -> Result<(), GroupDecodingError> {
    <Bn256 as Curve>::mul(input, output)
}

/// altbn_128 pairing
pub fn pairing(input: &[u8]) -> Result<bool, GroupDecodingError> {
    <Bn256 as Curve>::pairing(input, G1_LENGTH)
}

// #[test]
// fn test_alt_bn128_add() {
//     // zero-points additions
//     {
//         let input = hex!(
//             "
// 				0000000000000000000000000000000000000000000000000000000000000000
// 				0000000000000000000000000000000000000000000000000000000000000000
// 				0000000000000000000000000000000000000000000000000000000000000000
// 				0000000000000000000000000000000000000000000000000000000000000000"
//         );
//
//         let expected = hex!(
//             "
// 				0000000000000000000000000000000000000000000000000000000000000000
// 				0000000000000000000000000000000000000000000000000000000000000000"
//         );
//
//         assert_eq!(
//             &expected[..],
//             alt_bn128_add(&input[..]).expect("Builtin should not fail")
//         );
//     }
//
//     // no input, should not fail
//     {
//         let input = [0u8; 0];
//         let expected = hex!(
//             "
// 				0000000000000000000000000000000000000000000000000000000000000000
// 				0000000000000000000000000000000000000000000000000000000000000000"
//         );
//
//         assert_eq!(
//             &expected[..],
//             alt_bn128_add(&input[..]).expect("Builtin should not fail")
//         );
//     }
//
//     // should fail - point not on curve
//     {
//         let input = hex!(
//             "
// 				1111111111111111111111111111111111111111111111111111111111111111
// 				1111111111111111111111111111111111111111111111111111111111111111
// 				1111111111111111111111111111111111111111111111111111111111111111
// 				1111111111111111111111111111111111111111111111111111111111111111"
//         );
//
//         // let output = vec![0u8; 64];
//         let res = alt_bn128_add(&input[..]);
//         assert!(res.is_err(), "There should be built-in error here");
//     }
// }
// #[test]
// fn test_alt_bn128_mul() {
//     use hex_literal::hex;
//     // zero-point multiplication
//     {
//         let input = hex!(
//             "
// 				0000000000000000000000000000000000000000000000000000000000000000
// 				0000000000000000000000000000000000000000000000000000000000000000
// 				0200000000000000000000000000000000000000000000000000000000000000"
//         );
//
//         // let output = vec![0u8; 64];
//         let expected = hex!(
//             "
// 				0000000000000000000000000000000000000000000000000000000000000000
// 				0000000000000000000000000000000000000000000000000000000000000000"
//         );
//
//         assert_eq!(
//             &expected[..],
//             alt_bn128_scalar_mul(&input[..]).expect("Builtin should not fail")
//         );
//     }
//
//     // should fail - point not on curve
//     {
//         let input = hex!(
//             "
// 				1111111111111111111111111111111111111111111111111111111111111111
// 				1111111111111111111111111111111111111111111111111111111111111111
// 				0f00000000000000000000000000000000000000000000000000000000000000"
//         );
//
//         // let mut output = vec![0u8; 64];
//         let res = alt_bn128_scalar_mul(&input[..]);
//         assert!(res.is_err(), "There should be built-in error here");
//     }
// }
//
// #[test]
// fn add() {
//     use bn::{Fr, Group, G1};
//     use rand::{SeedableRng, StdRng};
//
//     let seed = [
//         0, 0, 0, 0, 0, 0, 64, 13, // 103245
//         0, 0, 0, 0, 0, 0, 176, 2, // 191922
//         0, 0, 0, 0, 0, 0, 0, 13, // 1293
//         0, 0, 0, 0, 0, 0, 96, 7u8, // 192103
//     ];
//
//     let p1 = G1::random(&mut StdRng::from_seed(seed));
//
//     println!("p1:{:?}", p1);
//     println!("p1 + p1:{:?}", p1 + p1);
//     println!("p1 * 2: {:?}", p1 * Fr::from_str("2").unwrap());
//
//     // let x = U256([
//     //     193057075356696845107056778628997597259,
//     //     58302148128148680438974674437016463407,
//     // ]);
//     // let y = U256([
//     //     97531101576300296613937243006504225138,
//     //     35484485583128177644927852191793807725,
//     // ]);
//     // let z = U256([
//     //     173294647386577742778337618982106380048,
//     //     54444350454913557390070431662659890840,
//     // ]);
//     let p1_2times = alt_bn128_scalar_mul(
//         &hex!("0230644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd46")[..],
//     );
//     println!("{:?}", p1_2times);
// }
