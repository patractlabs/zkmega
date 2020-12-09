//! Impl ops for curves
#![macro_use]

/// Paste pairing test
#[macro_export]
macro_rules! paste_test {
    () => {
        #[test]
        fn test_wasm_pairing() {
            let mut rng = test_rng();
            let a: G1Projective = rng.gen();
            let b: G2Projective = rng.gen();
            let s: Fr = rng.gen();

            let mut sa = a;
            sa.mul_assign(s);
            let mut sb = b;
            sb.mul_assign(s);

            let ans1 = <Bls12_377 as PairingEngine>::pairing(sa, b);
            let ans2 = <Bls12_377 as PairingEngine>::pairing(a, sb);
            let ans3 = <Bls12_377 as PairingEngine>::pairing(a, b).pow(s.into_repr());

            assert_eq!(ans1, ans2);
            assert_eq!(ans2, ans3);

            assert_ne!(ans1, Fq12::one());
            assert_ne!(ans2, Fq12::one());
            assert_ne!(ans3, Fq12::one());

            assert_eq!(ans1.pow(Fr::characteristic()), Fq12::one());
            assert_eq!(ans2.pow(Fr::characteristic()), Fq12::one());
            assert_eq!(ans3.pow(Fr::characteristic()), Fq12::one());
        }
    };
}
