use bellman_ce::{
    groth16::{
        create_random_proof, generate_random_parameters, prepare_verifying_key,
        verify_proof as raw_verify_proof,
    },
    pairing::{
        bn256::Bn256,
        ff::{PrimeField, PrimeFieldRepr},
        CurveAffine,
    },
    Circuit, ConstraintSystem, SynthesisError,
};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use megaclite::{
    parse::{proof_write, vk_write},
    raw_bn_bls::{alt_bn128::AltBn128, verify_proof},
};
use rand::thread_rng;
use rand::{Rng, SeedableRng, XorShiftRng};
use sapling_crypto_ce::circuit::{
    boolean::{AllocatedBit, Boolean},
    multipack,
    sha256::sha256,
};

struct Sha256Demo {
    input_data: Vec<u8>,
}

impl Circuit<Bn256> for Sha256Demo {
    fn synthesize<CS: ConstraintSystem<Bn256>>(
        self,
        mut cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        let mut h = Sha256::new();

        h.input(&self.input_data);
        let mut hash_result = [0u8; 32];
        h.result(&mut hash_result[..]);

        let mut foobar: Vec<Boolean> = [].to_vec();
        for (byte_i, input_byte) in self.input_data.into_iter().enumerate() {
            for bit_i in (0..8).rev() {
                let cs = cs.namespace(|| format!("input bit {} {}", byte_i, bit_i));
                foobar.push(
                    AllocatedBit::alloc(cs, Some((input_byte >> bit_i) & 1u8 == 1u8))
                        .unwrap()
                        .into(),
                );
            }
        }

        let r1 = &mut cs;
        let hash = sha256(r1, &foobar).unwrap();
        multipack::pack_into_inputs(cs, &hash)?;
        Ok(())
    }
}

fn eval_sha256(num_bytes: usize) {
    /// Proof production process
    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);
    let rng_foo = &mut thread_rng();

    let input_len = num_bytes;
    let data: Vec<u8> = (0..input_len).map(|_| rng.gen()).collect();

    println!("creating proving key");
    let params = {
        let c = Sha256Demo { input_data: data };

        generate_random_parameters(c, rng_foo).unwrap()
    };

    println!("creating verification key");
    let pvk = prepare_verifying_key(&params.vk);

    let more_data: Vec<u8> = (0..input_len).map(|_| rng.gen()).collect();
    let r1 = &more_data;
    let mut hasher = Sha256::new();
    hasher.input(r1);
    let mut hash_bytes: [u8; 32] = [0; 32];

    let r1 = &mut hash_bytes;
    hasher.result(r1);

    let more_c = Sha256Demo {
        input_data: more_data,
    };
    println!("constraints {:?} ", params.a.len());
    let start = std::time::SystemTime::now();
    let mut proof = create_random_proof(more_c, &params, rng_foo).unwrap();
    println!(
        "Prover time: {:?}",
        std::time::SystemTime::now().duration_since(start).unwrap()
    );

    let hash_bits = multipack::bytes_to_bits(r1);
    let inputs = multipack::compute_multipacking::<Bn256>(&hash_bits);

    /// Using our own verify_proof implementation to check the proof
    {
        // proof encode
        let mut proof_encode = vec![0u8; 32 * 8];
        proof_write(&mut proof, &mut proof_encode);
        // vk encode
        let mut vk_encode = vec![0u8; 32 * 14];
        vk_write(&mut vk_encode, &params);

        // vk_ic encode
        let vk_not_prepared = params.vk.ic.iter().map(|ic| ic.into_uncompressed().as_ref().to_vec()).collect::<Vec<_>>();
        let vk_ic = vk_not_prepared.iter().map(|ic| &ic[..]).collect::<Vec<_>>();

        // input encode

        let mut input = vec![[0u8; 32]; inputs.len()];
        inputs.iter().enumerate().for_each(|(i, scalar)| scalar.into_repr().write_be(&mut input[i][..]).unwrap());

        assert!(verify_proof::<AltBn128>(
            &*vk_ic,
            &*vk_encode,
            &*proof_encode,
            &input.iter().map(|x| &x[..]).collect::<Vec<_>>())
            .expect("verify_proof fail"));
    }

    /// Using bellman_ce verify_proof to check the proof
    assert!(raw_verify_proof(&pvk, &proof, &inputs).unwrap());
}

#[test]
fn test_sapling_sha256() {
    const NUM_HASHES: usize = 1;
    for i in 0..NUM_HASHES {
        let num_bytes = ((i + 1) * 64) - 9;
        println!("Hashing {:?} bytes", num_bytes);
        eval_sha256(num_bytes);
    }
}
