//! Groth16 verifaction

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use num_bigint::BigUint;
use parity_scale_codec::{Decode, Encode};

use crate::{
    error::{Error, Result},
    ops::CurveBasicOperations,
};

/// Groth16 Verifying Parcel
#[derive(Debug, Encode, Decode)]
pub struct Groth16Parcel {
    pub vk_gamma_abc: Vec<Vec<u8>>,
    pub vk: Vec<u8>,
    pub proof: Vec<u8>,
    pub public_inputs: Vec<Vec<u8>>,
}

/// Verify Wrapper
pub fn verify<C: CurveBasicOperations>(parcel: Vec<u8>) -> Result<bool> {
    let Groth16Parcel {
        vk_gamma_abc,
        vk,
        proof,
        public_inputs,
    } = Groth16Parcel::decode(&mut parcel.as_ref()).map_err(|_| Error::VerifyParcelFailed)?;
    verify_proof::<C>(vk_gamma_abc, vk, proof, public_inputs)
}

/// preprocess vk and proof to verify proof
pub fn preprocessed_verify_proof<C: CurveBasicOperations>(
    vk: [&str; 14],
    vk_gamma_abc: [&str; 6],
    proof_and_input: &[u8],
) -> Result<bool> {
    let bytes = hex::decode(proof_and_input).map_err(|e| format!("hex decode error:{}", e))?;
    let (proof, input) = bytes.split_at(2 * C::G1_LEN + C::G2_LEN);

    let mut vk_vec = Vec::new();
    vk_vec.append(&mut g2_pad_infinity(vk[6], vk[7], vk[8], vk[9]));
    vk_vec.append(&mut g2_pad_infinity(vk[10], vk[11], vk[12], vk[13]));
    vk_vec.append(&mut g1_pad_infinity(vk[0], vk[1]));
    vk_vec.append(&mut g2_pad_infinity(vk[2], vk[3], vk[4], vk[5]));

    verify_proof::<C>(
        (0..vk_gamma_abc.len() / 2)
            .map(|i| g1_pad_infinity(vk_gamma_abc[i * 2], vk_gamma_abc[i * 2 + 1]))
            .collect(),
        vk_vec,
        proof.to_vec(),
        (0..input.len() / C::SCALAR_LEN)
            .map(|i| input[i * C::SCALAR_LEN..(i + 1) * C::SCALAR_LEN].to_vec())
            .collect(),
    )
}

/// Groth16 verification
pub fn verify_proof<C: CurveBasicOperations>(
    vk_gamma_abc: Vec<Vec<u8>>,
    vk: Vec<u8>,
    proof: Vec<u8>,
    public_inputs: Vec<Vec<u8>>,
) -> Result<bool> {
    let g1_len = C::G1_LEN;
    let g2_len = C::G2_LEN;
    let g1_g2_len = C::G2_LEN + C::G1_LEN;
    let scalar_len = C::SCALAR_LEN;

    if (public_inputs.len() + 1) != vk_gamma_abc.len() {
        return Err(Error::VerifyParcelFailed);
    }

    // First two fields are used as the sum
    let mut acc = vk_gamma_abc[0].to_vec();

    // Compute the linear combination vk_x
    //  [(βui(x)+αvi(x)+wi(x))/γ] ∈ G1
    // acc = sigma(i:0~l)* [(βui(x)+αvi(x)+wi(x))/γ] ∈ G1
    for (i, b) in public_inputs.iter().zip(vk_gamma_abc.iter().skip(1)) {
        let mut mul_input = Vec::with_capacity(scalar_len + g1_len);
        mul_input.extend_from_slice(b);
        mul_input.extend_from_slice(i);

        // Check if invalid length
        if mul_input.len() != g1_len + scalar_len {
            return Err(Error::SerializeDataFailed);
            // return Err(format!(
            //     "Invalid input length {} for mul operation, should be {}",
            //     mul_input.len(),
            //     g1_len + scalar_len
            // )
            // .into());
        }
        let mul_ic = crate::call(0x01000001 + C::CURVE_ID, &mul_input)?;

        let mut acc_mul_ic = Vec::with_capacity(g1_len * 2);
        acc_mul_ic.extend_from_slice(acc.as_ref());
        acc_mul_ic.extend_from_slice(mul_ic.as_ref());

        // Check if invalid length
        if acc_mul_ic.len() != g1_len * 2 {
            return Err(Error::SerializeDataFailed);
        }
        acc = crate::call(0x01000000 + C::CURVE_ID, &*acc_mul_ic)?;
    }

    // The original verification equation is:
    // A * B = alpha * beta + acc * gamma + C * delta
    // ... however, we rearrange it so that it is:
    // A * B - acc * gamma - C * delta = alpha * beta
    // or equivalently:
    //    A   *    B    +  (-acc) * gamma +  (-C) * delta  +   (-alpha) * beta = 0
    let pairings = [
        (
            &proof[0..g1_len / 2],           // G1 x
            &proof[g1_len / 2..g1_len - 1],  // G1 y
            &proof[g1_len - 1..g1_len],      // G1 infinity
            &proof[g1_len..g1_len + g2_len], // G2
        ),
        (
            &acc[0..g1_len / 2],
            &*negate_y::<C>(&acc[g1_len / 2..g1_len - 1]),
            &acc[g1_len - 1..g1_len],
            &vk[0..g2_len],
        ),
        (
            &proof[g1_g2_len..g1_g2_len + g1_len / 2],
            &*negate_y::<C>(&proof[g1_g2_len + g1_len / 2..g1_g2_len + g1_len - 1]),
            &proof[g1_g2_len + g1_len - 1..g1_g2_len + g1_len],
            &vk[g2_len..g2_len * 2],
        ),
        (
            &vk[g2_len * 2..g2_len * 2 + g1_len / 2],
            &*negate_y::<C>(&vk[g2_len * 2 + g1_len / 2..g2_len * 2 + g1_len - 1]),
            &vk[g2_len * 2 + g1_len - 1..g2_len * 2 + g1_len],
            &vk[g2_len * 2 + g1_len..g2_len * 3 + g1_len],
        ),
    ];

    let mut input = Vec::with_capacity((g1_len + g2_len) * 4);
    pairings.iter().for_each(|(x, y, infinity, g2)| {
        input.extend_from_slice(x);
        input.extend_from_slice(y);
        input.extend_from_slice(infinity);
        input.extend_from_slice(g2);
    });

    // Return the result of computing the pairing check
    // e(p1[0], p2[0]) *  .... * e(p1[n], p2[n]) == 1.
    // For example pairing([P1(), P1().negate()], [P2(), P2()]) should return true.
    Ok(crate::call(0x01000002 + C::CURVE_ID, &input)?[0] == 0)
}

//
fn g1_pad_infinity(x: &str, y: &str) -> Vec<u8> {
    let mut bytes = vec![];
    bytes.append(&mut decode_hex(x.to_string()));
    bytes.append(&mut decode_hex(y.to_string()));
    bytes.push(0u8); // infinity flag
    bytes
}
fn g2_pad_infinity(x1: &str, y1: &str, x2: &str, y2: &str) -> Vec<u8> {
    let mut bytes = vec![];
    bytes.append(&mut decode_hex(x1.to_string()));
    bytes.append(&mut decode_hex(y1.to_string()));
    bytes.append(&mut decode_hex(x2.to_string()));
    bytes.append(&mut decode_hex(y2.to_string()));
    bytes.push(0u8); // infinity flag
    bytes
}
fn decode_hex(value: String) -> Vec<u8> {
    let mut bytes = hex::decode(value.strip_prefix("0x").unwrap()).unwrap();
    bytes.reverse();
    bytes
}

fn negate_y_based_curve(y: BigUint, MODULUS: &[u8]) -> BigUint {
    let q = BigUint::from_bytes_le(MODULUS);
    q.clone() - y % q
}

fn negate_y<C: CurveBasicOperations>(y: &[u8]) -> Vec<u8> {
    let neg_y = negate_y_based_curve(BigUint::from_bytes_le(y), C::MODULUS).to_bytes_le();

    // Because of randomness, Negate_y vector might not satisfy g1_y_len bytes.
    let mut neg_y_fill_with_zero = vec![0; y.len()];
    neg_y_fill_with_zero[0..neg_y.len()].copy_from_slice(&*neg_y);

    neg_y_fill_with_zero
}

#[test]
fn test_verify() {
    use crate::curve::Bls12_381;

    // VK = [alpha beta gamma delta]
    const VK:[&str;14] = ["0x0255430d4664c579165e2204f7dcef9556b8077fa7b0d9e6346f68e8a8028373e621e0070117ea963a28ea8daae7c295","0x08ed8bf7cd4fe9a8823594d2c99d6955228b46ee3ae1916ff7c087668eff3c078ee1942760d295af43b77492b100bdbd",
        "0x096325545fbbe733305b1d2b0034c01b3bdc7783b7cb9e995f18f14a88c43e2378052854b047c3ea0c941000265e7ea2", "0x00fb167465e35f0581d693aa7225821a0e077dcdd96e7c593415ce9dfa71874680bd9374a82bf729c4688b439f0ded3b", "0x0ebd72a410429e8ca4d379a335601f9d37208df1a067bf554f938f7336671371d86ca66aa83b60723782f3400b170413", "0x0377f821ed328dd127434c4fe06cc899711cdc47854a710102f51338e69bdbdea58ecbe4708d95a2edbecc1d8dbd9e57",
        "0x17c73570d12596a5e232f5240c10e9c2d1e7f4730746c8d4562309c0b7af3ad1c199317074e6d1a2e7cb15f1ee3334a1", "0x134256783454e4dad8631526d3b6f9ceaff12bb85e08693ef35cf6081683c4ab42e288d146b235899f50695c8e93493b", "0x0751498144a033087af7870423982200fef4fb15a115aef5120a5dcc573620af2e70bf83e6415ccc4c2b690a175b603b", "0x147b5b60d963b79c12e300a9764de5e2a1968cd07602597c2401a6b158bdc8a6bbad3dc9cc00d559d67ab33f199e032c",
        "0x02da9b61f2998b4e2a176d29244f188344a2af7ec66bdf61958344f9e533c731554fdcefb5801b7ad48a983abedd871d", "0x1906c82bfae3600f4d747fd10a8b2ef5219c0252a9ed85fbfbe2e2f126949450f34ada481a781b22db74f4d3d5ffd8eb", "0x14b54cf5402a7f4ae279c09ed9ea9b6aca2dcc96e5e677d8eed9edc3df8460581fca64c5593bd22f3c539a1d63517361", "0x15c6eb142f78a4b99caf9ca97237c2eb6f2dc83e52c395bf9223c4532d3dc754a3afa52ea4b9fef62954f9369218b90c"];
    const VK_GAMMA_ABC:[&str;6] =["0x07fb259c08d05a67e8e17ad3e6cb8e43a3ed1b10ece4dd8cbc0e3030016d09e43d195b7a34a8a46c76c4c327503c0b16","0x12cc86b8791137fef3b4caf4f4a38ec65eafef5bb34882d54ed07e7b1d8cf82423faec3d2c4029ae191119174bb17c4a","0x0d50f43997c6bc4a449d1e877436cbc9b5ecae45ee57ebcfe1a5a60787ce8ebb1692a3a23989d7e14e1216a959323ad3","0x0ec9f709318ddef4d513233c17e9112bdb8cc878bb86aef392dc1ddb4715e6f606ca8b122227ca5ecc31d39dbd8707c5","0x08171915be1b00c1a178f714bd5c3ea46c0070eabeb091a3cc124d53ef1dd47396c3ad2723aa91cd9c722582451282b4","0x1794ec6794a614392a3f63e165c7461cd6a00108ff3fcc102977de92ecb8588f557711f8ead71ffb681bc37b1ee1b7a6"];

    let proof_and_input = "c900f310725b3ec9dcc26021a8bc01558f44b7aa9bb6bc98399212652248d9988a3fceeccf07b1787cc7dea439c9b20683bc04a9e5a961a4094fc738f98f4878d5298587f9693bf18505b2cf0737fb49017775469d0215d1e0bd8fb36e36c81600f4f56a8745305c97ed023eaf97c5646ba67300a9430ae8ab437446d5f0fbaa48bcec410bbea6941131b518d1212faa02a032871dcedfd968bd0fc93c45c4e026f91193cb4910f92b98ef3f4fac9cf3d168a8fa338c90a4071a9c374edf80c008fbeb5a067bf5e50f213efc8da9822b5064666d369dd39dfd9199b4c2cb2273b5c4d3685216db4325429821bcde61330bfb5b8801e92cd231d53f2f3c09f39e54b99c3bd3780e9ba31b486f736243dd355406b4c5bc43dc65fd39688a41d8a30f0011333c5d8d51e3429ba5c60be670b62f078a048196b98bd5a890c71f1ff1c746d6d764f455d120ec484f87524c2c4b065e66bfa9ff4ab99f06df246ea397e5757bf5045fde4899d33821e2eb71ebcaac8274d306fc2620e2a88ce1d26f0e92030090000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000";

    assert_eq!(
        preprocessed_verify_proof::<Bls12_381>(VK, VK_GAMMA_ABC, proof_and_input.as_bytes())
            .unwrap(),
        true
    );
}
