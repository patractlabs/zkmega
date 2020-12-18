#![deny(
    unused_import_braces,
    unused_qualifications,
    trivial_casts,
    trivial_numeric_casts
)]
#![deny(unused_qualifications, variant_size_differences, stable_features)]
#![deny(
non_shorthand_field_patterns,
unused_attributes,
// unused_imports,
unused_extern_crates
)]
#![deny(
    renamed_and_removed_lints,
    stable_features,
    unused_allocation,
    unused_comparisons
)]
#![deny(
    unused_must_use,
    unused_mut,
    unused_unsafe,
    private_in_public,
    unsafe_code
)]

// For randomness (during paramgen and proof generation)
use rand::Rng;

// For benchmarking
// use std::time::{Duration, Instant};

// Bring in some tools for using pairing-friendly curves
// We're going to use the BLS12-377 pairing-friendly elliptic curve.
use ark_ff::{biginteger::BigInteger256, test_rng, Field, ToBytes};
// use ark_bls12_377::{Bls12_377, Fr};
// use ark_bls12_381::{Bls12_381, Fr};
use ark_bn254::{Bn254, Fr};
// use ark_bw6_761::{BW6_761,Fr};
// use ark_cp6_782::{CP6_782,Fr};

// We'll use these interfaces to construct our circuit.
use ark_ec::PairingEngine;
use ark_relations::{
    lc, ns,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError, Variable},
};
use arkworks::{groth16::verify_proof, CurveBasicOperations};
use num_bigint::BigUint;
use num_traits::Num;

const MIMC_ROUNDS: usize = 322;

/// This is an implementation of MiMC, specifically a
/// variant named `LongsightF322p3` for BLS12-377.
/// See http://eprint.iacr.org/2016/492 for more
/// information about this construction.
///
/// ``
/// function LongsightF322p3(xL ⦂ Fp, xR ⦂ Fp) {
///     for i from 0 up to 321 {
///         xL, xR := xR + (xL + Ci)^3, xL
///     }
///     return xL
/// }
/// ``
fn mimc<F: Field>(mut xl: F, mut xr: F, constants: &[F]) -> F {
    assert_eq!(constants.len(), MIMC_ROUNDS);

    for i in 0..MIMC_ROUNDS {
        let mut tmp1 = xl;
        tmp1.add_assign(&constants[i]);
        let mut tmp2 = tmp1;
        tmp2.square_in_place();
        tmp2.mul_assign(&tmp1);
        tmp2.add_assign(&xr);
        xr = xl;
        xl = tmp2;
    }

    xl
}

/// This is our demo circuit for proving knowledge of the
/// preimage of a MiMC hash invocation.
struct MiMCDemo<'a, F: Field> {
    xl: Option<F>,
    xr: Option<F>,
    constants: &'a [F],
}

/// Our demo circuit implements this `Circuit` trait which
/// is used during paramgen and proving in order to
/// synthesize the constraint system.
impl<'a, F: Field> ConstraintSynthesizer<F> for MiMCDemo<'a, F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        assert_eq!(self.constants.len(), MIMC_ROUNDS);

        // Allocate the first component of the preimage.
        let mut xl_value = self.xl;
        let mut xl =
            cs.new_witness_variable(|| xl_value.ok_or(SynthesisError::AssignmentMissing))?;

        // Allocate the second component of the preimage.
        let mut xr_value = self.xr;
        let mut xr =
            cs.new_witness_variable(|| xr_value.ok_or(SynthesisError::AssignmentMissing))?;

        for i in 0..MIMC_ROUNDS {
            // xL, xR := xR + (xL + Ci)^3, xL
            let ns = ns!(cs, "round");
            let cs = ns.cs();

            // tmp = (xL + Ci)^2
            let tmp_value = xl_value.map(|mut e| {
                e.add_assign(&self.constants[i]);
                e.square_in_place();
                e
            });
            let tmp =
                cs.new_witness_variable(|| tmp_value.ok_or(SynthesisError::AssignmentMissing))?;

            cs.enforce_constraint(
                lc!() + xl + (self.constants[i], Variable::One),
                lc!() + xl + (self.constants[i], Variable::One),
                lc!() + tmp,
            )?;

            // new_xL = xR + (xL + Ci)^3
            // new_xL = xR + tmp * (xL + Ci)
            // new_xL - xR = tmp * (xL + Ci)
            let new_xl_value = xl_value.map(|mut e| {
                e.add_assign(&self.constants[i]);
                e.mul_assign(&tmp_value.unwrap());
                e.add_assign(&xr_value.unwrap());
                e
            });

            let new_xl = if i == (MIMC_ROUNDS - 1) {
                // This is the last round, xL is our image and so
                // we allocate a public input.
                cs.new_input_variable(|| new_xl_value.ok_or(SynthesisError::AssignmentMissing))?
            } else {
                cs.new_witness_variable(|| new_xl_value.ok_or(SynthesisError::AssignmentMissing))?
            };

            cs.enforce_constraint(
                lc!() + tmp,
                lc!() + xl + (self.constants[i], Variable::One),
                lc!() + new_xl - xr,
            )?;

            // xR = xL
            xr = xl;
            xr_value = xl_value;

            // xL = new_xL
            xl = new_xl;
            xl_value = new_xl_value;
        }

        Ok(())
    }
}

#[test]
fn test_mimc_groth16() {
    // We're going to use the Groth-Maller17 proving system.
    use ark_groth16::{
        create_random_proof, generate_random_parameters, prepare_verifying_key,
        verify_proof as raw_verify_proof,
    };
    use rustc_hex::{FromHex, ToHex};

    // This may not be cryptographically safe, use
    // `OsRng` (for example) in production software.
    let rng = &mut test_rng();

    // Generate the MiMC round constants
    let constants = (0..MIMC_ROUNDS).map(|_| rng.gen()).collect::<Vec<_>>();

    println!("Creating parameters...");

    // Create parameters for our circuit
    let params = {
        let c = MiMCDemo::<Fr> {
            xl: None,
            xr: None,
            constants: &constants,
        };

        generate_random_parameters::<Bn254, _, _>(c, rng).unwrap()
    };

    // Prepare the verification key (for proof verification)
    let pvk = prepare_verifying_key(&params.vk);

    println!("Creating proofs...");

    // Let's benchmark stuff!
    // const SAMPLES: u32 = 50;

    // Just a place to put the proof data, so we can
    // benchmark deserialization.
    // let mut proof_vec = vec![];

    // Generate a random preimage and compute the image
    let xl: Fr = rng.gen();
    let xr: Fr = rng.gen();
    let image = mimc(xl, xr, &constants);

    let input_vec = vec![image.clone()];
    // proof_vec.truncate(0);

    {
        // Create an instance of our circuit (with the
        // witness)
        let c = MiMCDemo {
            xl: Some(xl),
            xr: Some(xr),
            constants: &constants,
        };

        // Create a groth16 proof with our parameters.
        let proof = create_random_proof(c, &params, rng).unwrap();

        // proof encode
        let mut proof_encode = Vec::new();
        proof.write(&mut proof_encode).unwrap();
        println!("proof:{}", proof_encode.to_hex::<String>());

        // vk encode
        let mut vk_encode = Vec::new();
        params.vk.gamma_g2.write(&mut vk_encode).unwrap();
        params.vk.delta_g2.write(&mut vk_encode).unwrap();
        params.vk.alpha_g1.write(&mut vk_encode).unwrap();
        params.vk.beta_g2.write(&mut vk_encode).unwrap();
        println!("vk:{}", vk_encode.to_hex::<String>());

        // vk_ic encode
        let vk_ic = params
            .vk
            .gamma_abc_g1
            .iter()
            .map(|ic| {
                let mut ic_vector = Vec::new();
                ic.write(&mut ic_vector).unwrap();
                ic_vector
            })
            .collect::<Vec<Vec<u8>>>();
        let mut vk_ic_slice = Vec::new();
        vk_ic.iter().for_each(|ic| vk_ic_slice.push(&ic[..]));
        println!("vk_ic:{}", vk_ic_slice[0].to_hex::<String>());
        println!("vk_ic2:{}", vk_ic_slice[1].to_hex::<String>());

        let mut input = vec![Vec::new(); 1];
        input_vec.iter().enumerate().for_each(|(i, scalar)| {
            scalar.write(&mut input[i]).unwrap();
        });

        let public_input = input.iter().map(|x| &x[..]).collect::<Vec<_>>();
        println!("public_input:{}", public_input[0].to_hex::<String>());

        assert!(verify_proof::<Bn254>(
            &vk_ic_slice,
            &vk_encode[..],
            &proof_encode[..],
            &public_input
        )
        .expect("verify proof fail "));
    }
}
