use crate::scratch::Curve;
use pairing_ce::{bls12_381::Bls12, GroupDecodingError};

const G1_LENGTH: usize = 64;

/// bls12_381 add
pub fn add(input: &[u8], output: &mut [u8]) -> Result<(), GroupDecodingError> {
    <Bls12 as Curve>::add(input, output)
}

/// bls12_381 mul
pub fn mul(input: &[u8], output: &mut [u8]) -> Result<(), GroupDecodingError> {
    <Bls12 as Curve>::mul(input, output)
}

/// bls12_381 pairing
pub fn pairing(input: &[u8]) -> Result<bool, GroupDecodingError> {
    <Bls12 as Curve>::pairing(input, G1_LENGTH)
}

// #[test]
// fn test_bls381_add() {
//     // Test identity add.
//     {
//         let a_hex = "400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
//         let a_uncompressed: Vec<u8> = a_hex.from_hex().unwrap();
//
//         let c_uncompressed =
//             bls381_add(a_uncompressed.repeat(2).as_ref()).expect("identity add failed");
//
//         let c = G1Affine::from_uncompressed(&c_uncompressed).unwrap();
//         assert!(bool::from(c.is_identity()));
//         assert!(bool::from(c.is_on_curve()));
//     }
//     {
//         let p1_hex = "17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb08b3f481e3aaa0f1a09e30ed741d8ae4fcf5e095d5d00af600db18cb2c04b3edd03cc744a2888ae40caa232946c5e7e1";
//         let p1_uncompressed: Vec<u8> = p1_hex.from_hex().unwrap();
//
//         let p1_add_p1 = bls381_add(&p1_uncompressed.repeat(2)[..]).expect("add fail:");
//
//         let p2_hex = "0572cbea904d67468808c8eb50a9450c9721db309128012543902d0ac358a62ae28f75bb8f1c7c42c39a8c5529bf0f4e166a9d8cabc673a322fda673779d8e3822ba3ecb8670e461f73bb9021d5fd76a4c56d9d4cd16bd1bba86881979749d28";
//         let p2_uncompressed: Vec<u8> = p2_hex.from_hex().unwrap();
//         assert_eq!(p2_uncompressed, p1_add_p1.to_vec());
//     }
// }
// #[test]
// fn test_bls381_mul() {}
// #[test]
// fn test_bls381_pairing() {}
