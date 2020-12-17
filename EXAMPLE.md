## use case

### mimc 
```
use zkp_u256::{U256, Zero};
use merkle_tree::mimc::{mimc,mimc_with_key}
let message = U256::from_decimal_str("49").unwrap();
let in_key = U256::zero();
assert_eq!(
    mimc(b"1"),
    mimc_with_key(vec![&message], &in_key)
);
```

### merkle_tree

```
use merkle_tree::MerkleTree;
let mut mt = MerkleTree::default();
let message = b"49";
let (leaf,index) = mt.insert(message).unwrap();
assert_eq!(mt.update(),mt.get_root());

let (merkle_proof,address_bits) = mt.get_proof(1);
assert!(mt.verify_merkle_proof(leaf,merkle_proof,address_bits));
```

### groth verifier
```
let rng = &mut thread_rng();
let constants = (0..MIMC_ROUNDS).map(|_| rng.gen()).collect::<Vec<_>>();

const SAMPLES: u32 = 10;
let mut proof_vec = vec![];


println!("Creating parameters...");
let num_repetitions = ((sample_idx as usize) + 1) * MIMC_STEP;

let params = {
    let c = MiMCDemo::<Bls12> {
        repetitions: num_repetitions,
        x: None,
        k: None,
        constants: &constants,
    };

    generate_random_parameters(c, rng).unwrap()
};

let pvk = prepare_verifying_key(&params.vk);

println!("Creating proofs...");

let x = rng.gen();
let k = rng.gen();

let mut input_vec = vec![];

for _ in 0..num_repetitions {
    input_vec.push(mimc::<Bls12>(x, k, &constants));
}
proof_vec.truncate(0);

let c = MiMCDemo {
    repetitions: num_repetitions,
    x: Some(x),
    k: Some(k),
    constants: &constants,
};

{
    // Create a groth16 proof with our parameters.
    let proof = create_random_proof(c, &params, rng).unwrap();

    proof.write(&mut proof_vec).unwrap();
}


let mut proof = Proof::read(&proof_vec[..]).unwrap();

/// check the proof

// proof encode
let mut proof_encode = vec![0u8; 48 * 8];
proof_write(&mut proof, &mut proof_encode);
// vk encode
let mut vk_encode = vec![0u8; 48 * 14];
vk_write(&mut vk_encode, &params);

// vk_ic encode
let vk_not_prepared = params
    .vk
    .ic
    .iter()
    .map(|ic| ic.into_uncompressed().as_ref().to_vec())
    .collect::<Vec<_>>();
let vk_ic = vk_not_prepared.iter().map(|ic| &ic[..]).collect::<Vec<_>>();

// input encode
let mut input = vec![[0u8; 32]; input_vec.len()];
input_vec.iter().enumerate().for_each(|(i, scalar)| {
    scalar.into_repr().write_le(&mut input[i][..]);
});
let public_input = &input.iter().map(|x| &x[..]).collect::<Vec<_>>();

let start = Instant::now();
/// test verify_proof that based on the Bls12_381 curve.
assert!(verify::<Bls12>(&*vk_ic, &*vk_encode, &*proof_encode,public_input)
            .expect("verify_proof fail"));
```
