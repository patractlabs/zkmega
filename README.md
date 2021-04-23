# zkMega

[zkMega](https://patractlabs.github.io/substrate-contracts-book/zh_CN/zkmega/introduction.html) is a zero-knowledge proof tool set building for the Polkadot ecology.

* Element group for discussion: https://app.element.io/#/room/#PatractLabsDev:matrix.org

## ZK Rollup Introduction

Compared with the privacy function, the performance improvement brought by Rollup is the
early application direction of zero-knowledge proof. At present, the Layer 2 expansion
plan of the blockchain is to transfer a considerable part of the on-chain workload to
off-chain to complete, and the most watched one is ZK Rollup. The essence of ZK Rollup
is to compress the application on-chain state and store it in a Merkle tree, and move
the state transition funtions to off-chain. At the same time, the correctness of the
off-chain state transition process is guaranteed through the proof of zkSNARK. Compared
with the high cost of directly processing state changes on the chain, the ZK Proof's
on-chain smart contract verification is extremely cost low. At the same time, the
compressed information will also be submitted to the chain together with the proof,
which ensures data availability and obtains the same level of security as Layer 1.

The Ethereum Layer 2 protocols related to ZK Rollup are: [zkSync][zkSync], [aztec][aztec],
etc. Their contract verification modules share a part of the elliptic curve's basic algorithms.
In 2017, Ethereum integrated three basic cryptographic calculation units of the alt
bn128 curve in the form of pre-compiled contracts, which are [EIP196][EIP196]’s ADD and Scalar_MUL
algorithms, and [EIP197][EIP197]’s Pairing algorithm. On top of this, due to the lack of rapid
upgrade capabilities of Ethereum, the community can only encapsulate some  tool libraries
through costly Solidity contracts. On top of these basic contract  libraries, many DApps can combine
ZK Rollup technology to achieve some innovations, such as [loopring][loopring], [gitcoin][gitcoin]
and [uniswap][uniswap] etc. However, in the past 3 years, ZK technology has further developed,
such as the more practical [BLS curve][BLS curve], and [PLONK algorithm][PLONK algorithm] etc.
Ethereum has not yet supported it.

## Summary of zkMega's  plan
- v0.1: Provide on-chain support for elliptic curve alt_bn128 、 bls12_381 、 BLS12-377 and BW6_761
    - Integrate addition (ADD), scalar multiplication (MUL) and Pairing functions of the curves in Native layer and Runtime WASM layer.
    - Provide these three functions to the upper Runtime Pallets and Contracts to call.
    - In the Runtime layer and the Ink! contract layer, provide two zkSNARK Verify upper-layer interfaces ( verification function of groth16, similar to the Verifier library of ethsnarks).
    - Start the Metis project and implement EdDSA, MerkleTree, MiMC Hash, etc. contract library on the Ink! contract layer.

- v0.2: Provide off-chain toolbox support for Ink! contract
    - ZoPatract_core integrates arkworks-g16、arkworks-bls12_381、arkworks-bn254
    - Implement CLI command for the whole phase of Zksnarks agreement (Zopatract_cli)
    - The Ink smart contract template ink_verifier.rs that implements the verifier function is connected to Zopatract_cli's export-verifier.
  - Modify the Zopatract JavaScript toolkit
  - Test CLI's use of Arkworks groth16 algorithm and curve Bls12_381 and Bn254 curve package
  - Test CLI on compile, setup, compute-witness, generate-proof, export-verifier commands in ink! smart contract environments.
- v0.3: Create a sample payment DApp based on Megaclite

##  Current Development Progress
 At present, we have completed the 0.2 version development.

You Can view:
 - 0.1 
    + [Pairing](https://patractlabs.github.io/zkmega/pairing)
    + [Pallet Contracts](https://patractlabs.github.io/zkmega/pallet-contracts)
    + [Metis](https://patractlabs.github.io/zkmega/metis)

     Polkadot Treasury report for v0.1: https://polkadot.polkassembly.io/post/221.

- 0.2 
    + [ZoPatract](https://github.com/patractlabs/ZoPatract/)

    Report  for v0.2 : https://github.com/patractlabs/ZoPatract/blob/master/REPORT.md

## LICENSE

Apache-2.0

[book]: https://patractlabs.github.io/substrate-contracts-book/zh_CN/zkmega/introduction.html
[zkSync]: https://zksync.io/
[aztec]: https://aztec.network/
[EIP196]: https://github.com/ethereum/EIPs/blob/master/EIPS/eip-196.md
[EIP197]: https://github.com/ethereum/EIPs/blob/master/EIPS/eip-197.md
[gitcoin]: https://gitcoin.co/
[uniswap]: https://uniswap.org/
[loopring]: https://loopring.org/
[BLS curve]: https://electriccoin.co/blog/new-snark-curve/
[PLONK algorithm]: https://eprint.iacr.org/2019/953/20190827:165656
