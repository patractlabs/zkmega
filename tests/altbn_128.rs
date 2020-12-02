// #[test]
// fn test_add() {
//     // use pairing_ce::{
//     //     bn256::{G1Affine, G1Uncompressed},
//     //     CurveAffine, EncodedPoint,
//     // };
//     // zero-points additions
//     {
//         // let mut output = [0; 64];
//         // let input = hex!(
//         //     "
//         // 		0000000000000000000000000000000000000000000000000000000000000000
//         // 		0000000000000000000000000000000000000000000000000000000000000000
//         // 		0000000000000000000000000000000000000000000000000000000000000000
//         // 		0000000000000000000000000000000000000000000000000000000000000000"
//         // );
//         //
//         // let expected = hex!(
//         //     "
//         // 		0000000000000000000000000000000000000000000000000000000000000000
//         // 		0000000000000000000000000000000000000000000000000000000000000000"
//         // );
//         //
//         // add(&input[..], &mut output).expect("Builtin should not fail");
//
//         // assert_eq!(&expected[..], output);
//         // let input = [0; 128];
//         // let (mut p1, mut p2) = (G1Uncompressed::empty(), G1Uncompressed::empty());
//         // let p1 = G1Uncompressed::empty();
//         // p1.into_affine().unwrap();
//
//         // p1.as_mut().copy_from_slice(&input[0..64]);
//         // p2.as_mut().copy_from_slice(&input[64..]);
//
//         // println!("{:?}", p1.into_affine_unchecked().unwrap());
//         // println!("{:?}", G1Affine::zero());
//     }
//
//     // no input, should not fail
//     // {
//     //     let mut output = [0; 64];
//     //     let input = [0u8; 0];
//     //     let expected = hex!(
//     //         "
//     // 			0000000000000000000000000000000000000000000000000000000000000000
//     // 			0000000000000000000000000000000000000000000000000000000000000000"
//     //     );
//     //
//     //     add(&input[..], &mut output).expect("Builtin should not fail");
//     //     assert_eq!(&expected[..], output);
//     // }
//
//     // should fail - point not on curve
//     // {
//     //     let mut output = [0; 64];
//     //     let input = hex!(
//     //         "
//     // 			1111111111111111111111111111111111111111111111111111111111111111
//     // 			1111111111111111111111111111111111111111111111111111111111111111
//     // 			1111111111111111111111111111111111111111111111111111111111111111
//     // 			1111111111111111111111111111111111111111111111111111111111111111"
//     //     );
//     //
//     //     let res = add(&input[..], &mut output);
//     //     assert!(res.is_err(), "There should be built-in error here");
//     // }
// }

// #[test]
// fn test_mul() {
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
//             mul(&input[..]).expect("Builtin should not fail")
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
//         let res = mul(&input[..]);
//         assert!(res.is_err(), "There should be built-in error here");
//     }
// }

// #[test]
// fn test_add() {
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
//     let p1_2times = scalar_mul(
//         &hex!("0230644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd46")[..],
//     );
//     println!("{:?}", p1_2times);
// }
use bellman_ce::{
    groth16::{
        create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof, Proof,
    },
    pairing::{
        bn256::Bn256,
        ff::{Field, PrimeField, PrimeFieldRepr},
        CurveAffine, Engine,
    },
    Circuit, ConstraintSystem, SynthesisError,
};
use std::time::Instant;

use megaclite::{
    altbn_128::bn256_verify_proof,
    parse::{proof_write, vk_write},
};
use rand::{thread_rng, Rng};

const MIMC_ROUNDS: usize = 1;
const MIMC_STEP: usize = 1;

/// This is an implementation of MiMC, specifically a
/// variant named `LongsightF322p3` for BLS12-381.
/// See http://eprint.iacr.org/2016/492 for more
/// information about this construction.
///
/// ```
/// function LongsightF322p3(xL ⦂ Fp, xR ⦂ Fp) {
///     for i from 0 up to 321 {
///         xL, xR := xR + (xL + Ci)^3, xL
///     }
///     return xL
/// }
/// `
fn mimc<E: Engine>(mut x: E::Fr, k: E::Fr, constants: &[E::Fr]) -> E::Fr {
    assert_eq!(constants.len(), MIMC_ROUNDS);

    for i in 0..MIMC_ROUNDS {
        // tmp1 = x + k + c[i]
        let mut tmp1 = x;
        tmp1.add_assign(&constants[i]);
        tmp1.add_assign(&k);
        // tmp2 = (x + k + c[i])^2
        let mut tmp2 = tmp1;
        tmp2.square();
        // tmp3 = (x + k + c[i])^4
        let mut tmp3 = tmp2;
        tmp3.square();
        // tmp4 = (x + k + c[i])^6
        let mut tmp4 = tmp3;
        tmp4.mul_assign(&tmp2);
        // tmp5 = (x + k + c[i])^7
        let mut tmp5 = tmp4;
        tmp5.mul_assign(&tmp1);
        x = tmp5;
    }

    x
}

struct MiMCDemo<'a, E: Engine> {
    repetitions: usize,
    x: Option<E::Fr>,
    k: Option<E::Fr>,
    constants: &'a [E::Fr],
}

impl<'a, E: Engine> Circuit<E> for MiMCDemo<'a, E> {
    fn synthesize<CS: ConstraintSystem<E>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        assert_eq!(self.constants.len(), MIMC_ROUNDS);

        for _ in 0..(self.repetitions) {
            let mut x_value = self.x;
            let mut x = cs.alloc(
                || "preimage x",
                || x_value.ok_or(SynthesisError::AssignmentMissing),
            )?;

            // Allocate the key.
            let k_value = self.k;
            let k = cs.alloc(
                || "preimage key",
                || k_value.ok_or(SynthesisError::AssignmentMissing),
            )?;

            for i in 0..MIMC_ROUNDS {
                // x := (x + k + Ci)^7
                let cs = &mut cs.namespace(|| format!("round {}", i));

                let tmp_value = x_value.map(|mut e| {
                    e.add_assign(&k_value.unwrap());
                    e.add_assign(&self.constants[i]);
                    e.square();
                    e
                });
                let tmp = cs.alloc(
                    || "tmp",
                    || tmp_value.ok_or(SynthesisError::AssignmentMissing),
                )?;

                cs.enforce(
                    || "tmp = (x + k + Ci)^2",
                    |lc| lc + x + k + (self.constants[i], CS::one()),
                    |lc| lc + x + k + (self.constants[i], CS::one()),
                    |lc| lc + tmp,
                );

                // tmp_2 = (x + k + ci)^4
                // tmp_2 = tmp^2
                let tmp_2_value = tmp_value.map(|mut e| {
                    e.mul_assign(&tmp_value.unwrap());
                    e
                });
                let tmp_2 = cs.alloc(
                    || "tmp2",
                    || tmp_2_value.ok_or(SynthesisError::AssignmentMissing),
                )?;
                cs.enforce(
                    || "tmp2 = (xL + Ci)^4",
                    |lc| lc + tmp,
                    |lc| lc + tmp,
                    |lc| lc + tmp_2,
                );

                // tmp_3 = (x + k + ci)^6
                // tmp_3 = (tmp_2)(tmp)
                let tmp_3_value = tmp_2_value.map(|mut e| {
                    e.mul_assign(&tmp_value.unwrap());
                    e
                });
                let tmp_3 = cs.alloc(
                    || "tmp3",
                    || tmp_3_value.ok_or(SynthesisError::AssignmentMissing),
                )?;
                cs.enforce(
                    || "tmp3 = (xL + Ci)^6",
                    |lc| lc + tmp_2,
                    |lc| lc + tmp,
                    |lc| lc + tmp_3,
                );

                // new_x = (x + k + ci)^7
                // new_x = (x + k + ci).(tmp_3)
                let rhs_value = x_value.map(|mut e| {
                    e.add_assign(&k_value.unwrap());
                    e.add_assign(&self.constants[i]);
                    e
                });
                let new_x_value = tmp_3_value.map(|mut e| {
                    e.mul_assign(&rhs_value.unwrap());
                    e
                });
                let new_x = if i == (MIMC_ROUNDS - 1) {
                    cs.alloc_input(
                        || "image",
                        || new_x_value.ok_or(SynthesisError::AssignmentMissing),
                    )?
                } else {
                    cs.alloc(
                        || "new_x",
                        || new_x_value.ok_or(SynthesisError::AssignmentMissing),
                    )?
                };

                cs.enforce(
                    || "new_x = (x + k + Ci)^7",
                    |lc| lc + x + k + (self.constants[i], CS::one()),
                    |lc| lc + tmp_3,
                    |lc| lc + new_x,
                );

                x = new_x;
                x_value = new_x_value;
            }
        }
        Ok(())
    }
}

#[test]
fn test_mimc() {
    // This may not be cryptographically safe, use
    // `OsRng` (for example) in production software.
    let rng = &mut thread_rng();

    // Generate the MiMC round constants
    let constants = (0..MIMC_ROUNDS).map(|_| rng.gen()).collect::<Vec<_>>();

    // Let's benchmark stuff!
    const SAMPLES: u32 = 10;
    // Just a place to put the proof data, so we can
    // benchmark deserialization.
    let mut proof_vec = vec![];

    for sample_idx in 0..SAMPLES {
        println!("Creating parameters...");
        let num_repetitions = ((sample_idx as usize) + 1) * MIMC_STEP;

        // Create parameters for our circuit
        let params = {
            let c = MiMCDemo::<Bn256> {
                repetitions: num_repetitions,
                x: None,
                k: None,
                constants: &constants,
            };

            generate_random_parameters(c, rng).unwrap()
        };

        // Prepare the verification key (for proof verification)
        let pvk = prepare_verifying_key(&params.vk);

        println!("Creating proofs...");

        // Generate a random preimage and compute the image
        let x = rng.gen();
        let k = rng.gen();

        let mut input_vec = vec![];

        for _ in 0..num_repetitions {
            input_vec.push(mimc::<Bn256>(x, k, &constants));
        }
        println!("{}", input_vec.len());
        proof_vec.truncate(0);

        let c = MiMCDemo {
            repetitions: num_repetitions,
            x: Some(x),
            k: Some(k),
            constants: &constants,
        };

        let start = Instant::now();
        {
            // Create a groth16 proof with our parameters.
            let proof = create_random_proof(c, &params, rng).unwrap();

            proof.write(&mut proof_vec).unwrap();
        }

        let total_proving = start.elapsed();
        let start = Instant::now();

        let mut proof = Proof::read(&proof_vec[..]).unwrap();

        /// Using our own verify_proof implementation to check the proof
        {
            // proof encode
            let mut proof_encode = vec![0u8; 32 * 8];
            proof_write(&mut proof, &mut proof_encode);
            // vk encode
            let mut vk_encode = vec![0u8; 32 * 14];
            vk_write(&mut vk_encode, &params);

            // vk_ic encode
            let vk_not_prepared = params.vk.ic.iter()
                .map(|ic| ic.into_uncompressed().as_ref().to_vec())
                .collect::<Vec<_>>();
            let vk_ic = vk_not_prepared.iter().map(|ic| &ic[..]).collect::<Vec<_>>();

            // input encode
            let mut input = vec![[0u8; 32]; input_vec.len()];
            input_vec.iter().enumerate().for_each(|(i, scalar)| {
                scalar.into_repr().write_be(&mut input[i][..]).unwrap();
            });
            let public_input = &input.iter().map(|x| &x[..]).collect::<Vec<_>>();
            println!("{:?}", input);
            println!("{:?}", input[0].len());

            // TODO: There is an error that needs to be fixed
            assert_eq!(bn256_verify_proof(
                &*vk_ic, &*vk_encode,
                &*proof_encode, public_input)
                    .expect("verify_proof fail"),
            false);
        }

        // Check the proof
        assert!(verify_proof(&pvk, &proof, &input_vec).unwrap());

        let total_verifying = start.elapsed();

        let proving_avg = total_proving;
        let proving_avg =
            proving_avg.subsec_nanos() as f64 / 1_000_000_000f64 + (proving_avg.as_secs() as f64);

        let verifying_avg = total_verifying;
        let verifying_avg = verifying_avg.subsec_nanos() as f64 / 1_000_000_000f64
            + (verifying_avg.as_secs() as f64);

        println!("applying MiMC cipher: {:?} times", num_repetitions);
        println!("proving time: {:?} seconds", proving_avg);
        println!("verifying time: {:?} seconds", verifying_avg);
    }
}
