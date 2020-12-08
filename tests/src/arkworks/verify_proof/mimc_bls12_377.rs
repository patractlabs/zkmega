use super::*;
use ark_bls12_377::{Bls12_377, Fr};

#[test]
pub fn test_bls12_377_groth16() {
    test_mimc_groth_17::<Bls12_377>();
}

#[test]
pub fn test_bls12_377_verify() {
    let proof = "e2342d048b06618cfc80a5fb5301373032d5780ca45e5287e824fd1d3dccfc36d1befe73edaea7378968a003f94e180190f4ad7dc16060ebfbe2bff08077e404e878850682e15f64a5b0f243f53a6d61dc358d41fa6aed615c6296904d71c200c04fe75b3adbecdc3d00df22556a1c949e4b87c81483225f6eaa7560e3035a1c919c4a0d446125dca2e2ef541d58a20026b3bb740a34b4db3a1c689d471b5a7834bb27fd57108bca8cd88d12bb81eae5f8ad807e5de758978ab51d0146017180";
    let vk = "5720823639fe8f3818ade6c24c6d6ed8f78b87369a9fac8a548e01ca001e424ba931033958641fe06677c3f4a1b39f819d629094c47bdf026bbd3424f6e7748b0c3529b495cf24677f309d9f1ed7d3435513224cba204902fd733522b3b855015f395e5b2a7f0fe9e50665d705c48f9b8b20a3f9619f50c1dcddbea7c4a561cdffed750045f875b2d2c569e8d5d0328054f796d78550d14c4ee03cd5915fd8037569696f878312b0093f0149919fc717c0b3f8a2d419fad0edde7e1d55cd2f002c312b660250225a00baf5df511dcce61eb23b28f3980be7fcde0091284670e9064015f452f244e65cad047a20fdd080f70cb4191a256ed9a4d90305f5864993032a0e40433040c259b14c8069d117f3491d75d993271a0a3f13efdd5b18ce006d90eff3b45f8fa10b43a2d4fb80a554304ba5d94ddc19c92b2e1e2f756bd3042c4c0d6e10b6c22c2ea5eb136fb42980020000000000000071c5a779bb19e0aeb6030768f5001017bbd4edb645306ff1591424c501e7aa8b480fc18d8ed404c893d473afa9b82c014d0869345487bb4070110fa318b0f408dcffce12304589878797615d8a72cc00b6a1e3f50cc0e5977063864009072681";
    let image = "a4961fec1c734055a80af91b3fd7405a2cdb741e1867aa7f783ca0959a077110";
    let proof_input: Vec<u8> = FromHex::from_hex(proof).unwrap();
    let vk_input: Vec<u8> = FromHex::from_hex(vk).unwrap();
    let image_input: Vec<u8> = FromHex::from_hex(image).unwrap();

    let proof = Proof::<Bls12_377>::deserialize(&*proof_input).expect("Proof deserialize fail:");
    let vk = VerifyingKey::<Bls12_377>::deserialize(&*vk_input).expect("vk deserialize fail");
    let pvk = PreparedVerifyingKey::<Bls12_377>::from(vk);
    let image = Fr::read(&*image_input).unwrap();

    let start = Instant::now();
    assert!(verify_proof(&pvk, &proof, &[image]).unwrap());
}
