# Milestones

### M1: Integrate the basic units of alt_bn128 & bls12_381 (3 developer * 1 weeks)
1. Prepare the 3 basic algorithm codes (Add, Scalar_Mul, and Pairing) of alt_bn128 elliptic curve from zcash's official library [bn](https://github.com/zcash-hackworks/bn).
2. Prepare the same 3  basic algorithm codes of bls12_381 from zcash's official library [bls12_381](https://github.com/zkcrypto/bls12_381) .
3. Integrate the above 6 algorithms into the under layer of Substrate Runtime.

### M2: Provide to upper runtime and smart contract applications (3 developers * 1 weeks)
1. Add Runtime_interface to the Jupiter testnet and provide it to Runtime applications.
2. Add Contract_Seal to the contract module of the Jupiter testnet and provide it to the WASM contract application.

### M3: Integrate upper-level verification and tool functions (3 developers * 1 week)
1. The runtime pallet that implements the zkSNARK verification algorithm.
2. The Metis contract library that implements the zkSNARK verification algorithm.
3. The Metis contract library that implements other commonly used algorithms, such as Poseidon, EdDSA, MerkleTree, MiMC and other commonly used algorithms.

### M4: Test (3 developers * 1 week)
1. Test six basic elliptic curve algorithms
2. Test the zkSNARK verification algorithm of the Runtime layer
3. Test the zkSNARK verification algorithm implemented in the contract layer
4. Test Poseidon, EdDSA, MerkleTree, MiMC and other commonly used algorithm contract templates.

### M5: Benchmark (3 developers * 1 week)
1. Provide benchmark results of all the above functions and develop them for the community as a reference for transaction weight and pricing.
