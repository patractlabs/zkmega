use super::*;
use ark_bn254::{Bn254, Fr};

#[test]
pub fn test_bn254_groth16() {
    test_mimc_groth_17::<Bn254>();
}

#[test]
pub fn test_bn254_verify() {
    let proof = "cf3bb657065cd0878eabc53790c5fa3680cad24db586bab799e3ae790217c61b0227ad22191396e6ffe7aff14de91d7801e943f836de399b5fd5c8312086440c036648b695ffa27790f2f2a63fcf4c0b130e6b88ac786e3351fc6f13452d18837a922013b86567d6d48ff65baf8cdd0558c52db98f27b6c216df96f8aa1d4d2d";
    let vk = "cc165291944a7b617bad2e6f1560f89f4e7eb42f154333b730e2f2da7828919d94b31d9ebcca3957fff3d94960b0eba2197fabaaa68776e767e8785f63b4541d456d755240d72dae2ee0452c0cd705d7312b9a1d17cff916e87c763d459a5504f00c5d68983d7c029a66ae498a1cc6a2423bf67760bbf922ea7a937a87705e158db79bbff4eb9516d5342ce8cd8d09f68fa094434edceebbb43383fc415d9b23f70bfb6ec84456af219bcd1d682133db3db80b0ae54cf31879e74a0ae547ec0ef92f56757f4a7976c50dc3d538cb1b4ed1736d4e8e8734d5cbf85b2ac8a93d0b020000000000000003c2e806d904d8789f0caf1276fc4897bbf2bbb2e7243df2e4bf941ce61679844d609323ed564943ba7dfd3b362cc85abf7691dd9957c613fc0d282f258f4d17";
    let image = "829d3d572251103bac02b39e397c657c0b1f372bfc4d1b74f6c23de4993b4824";

    let proof_input: Vec<u8> = FromHex::from_hex(proof).unwrap();
    let vk_input: Vec<u8> = FromHex::from_hex(vk).unwrap();
    let image_input: Vec<u8> = FromHex::from_hex(image).unwrap();

    let proof = Proof::<Bn254>::deserialize(&*proof_input).expect("Proof deserialize fail:");
    let vk = VerifyingKey::<Bn254>::deserialize(&*vk_input).expect("vk deserialize fail");
    let pvk = PreparedVerifyingKey::<Bn254>::from(vk);
    let image = Fr::read(&*image_input).unwrap();

    let start = Instant::now();
    assert!(verify_proof(&pvk, &proof, &[image]).unwrap());
}
