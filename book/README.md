# Megaclite

Megaclite is a zero-knowledge proof tool set building for the Polkadot ecology. 

## Curves

Here we provide `alt_bn128` and `bls12_381` curves for ZKP, both of them contain `Integrate addition`, 
`Scalar multiplication` and `Pairing` functions, for helping devlopers trying and testing the curves, 
we provide `groth16` for verifaction as well.


## Runtime Interfaces

+ [jupiter-contracts][jupiter-contracts]


## Ink!

+ [jupiter-ink-env][jupiter-ink-env]


## References

+ [alt\_bn128][alt_bn128]
+ [bls12\_381][bls12_381]
+ [groth16][groth16]

[alt_bn128]: https://github.com/zcash-hackworks/bn
[bls12_381]: https://github.com/zkcrypto/bls12_381
[groth16]: http://www.zeroknowledgeblog.com/index.php/groth16
[jupiter-contracts]: https://github.com/patractlabs/substrate/tree/contracts/frame/contracts
[jupiter-ink-env]: https://github.com/patractlabs/ink/tree/altbn128/env/crates/env
