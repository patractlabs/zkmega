use super::*;
use ark_bls12_381::{Bls12_381, Fr};

#[test]
pub fn test_bls12_388_groth16() {
    test_mimc_groth_17::<Bls12_381>();
}

#[test]
pub fn test_bls12_388_verify() {
    let proof = "fcafa9753aee681f006d7bf6fc885dcc1a70c33bbfe92bb581ac51e1325b782b1128b1586767cb2ea27555e243abd293443cc8d6ffec17c44207dd41e8ab2f2f2c663f5ed9ab724321de43cbe2ea81908c13643c42a41be1adbc5076e4927b13046d3cdaf5ef775ee48d842599508178276cd41d30e289cbd1d2bb9f92f59b520138fef3e9529139f3961f40f6ea9211de094abc5e25fac259efcfd79a97401359069cde3b48eaa9d3a4d848959eb269d788a415d9c22fb418459474f1f0fc96";
    let vk = "408697568bc15871da494a2fe12199531130a8a5b96f07c69020dc01bf9f3ed043a6e1bd08f8d7826c7d5c0fc5c771995b6d86316ea7c1dc4df3e3396a9f78aadf416d5af1619883f5dbbf0a39c6743b194e5adedda54d7cb3ae33e556602e19dd43a31bca7bdb15ad5245dc1ad0affb97edc8f35bb00d1c5b417a132d5be9b72a39870193b6a142198a888c0fe3768f9094c864d37f890720969430f2682ccc76dc83f9234694590bd334b40dbfa7f28de40e914571d8ae2a88712844f86f0d6dd2f4875120f66304bfcedd55c9a805310155a5345ddc626cbb6fe5ce7026be174a08c798e4d1f1010f97138c30b10a0c06c5cc02155140b97283cc024daf57023a781a9115bb6e17350264f8b52784a6008921402af8c05dd39f4b789bd50435d2c5aefeb6dc9817af41f45779e3a2d80ab98ef923723b938566747c10a4a169ca5097c5d5073449c3d2e65e1629980200000000000000e60f6032ed7f170ba3481c5d99c0890554c808ad380a7c9aefc9e62eeecf0363ecef5e59ba8eaf6dd0af40e08d7cfd82ea7f398187aecf2721441218e54645b17acd35bdf65886255e772a58353ef2e25c92cf7f872bea5afca6081d03e6b115";
    let image = "1d6c7850edbac8a5281ab93d2ed245d47b64f20c21950926d595624b488c291c";

    let proof_input: Vec<u8> = FromHex::from_hex(proof).unwrap();
    let vk_input: Vec<u8> = FromHex::from_hex(vk).unwrap();
    let image_input: Vec<u8> = FromHex::from_hex(image).unwrap();

    let proof = Proof::<Bls12_381>::deserialize(&*proof_input).expect("Proof deserialize fail:");
    let vk = VerifyingKey::<Bls12_381>::deserialize(&*vk_input).expect("vk deserialize fail");
    let pvk = PreparedVerifyingKey::<Bls12_381>::from(vk);
    let image = Fr::read(&*image_input).unwrap();

    let start = Instant::now();
    assert!(verify_proof(&pvk, &proof, &[image]).unwrap());
}
