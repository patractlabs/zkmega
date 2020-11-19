## Pairing-Friendly Curves

> In Progress

Here we provide [alt\_bn128][alt_bn128] and [bls12\_381][bls12_381] curves for the 
[pairing-friendly curves][ietf], both of them contain `add`, `mul` and `pairing` functions, 
for helping devlopers trying and testing the curves, we provide [groth16][groth16] for
`verifaction`.


## Interfaces

We follow the format of the contracts that ethereum pre-compiled.

```rust,ignore
/// Pairing-Friendly Curves
trait PairingOp {
    type VK;
    type Proof;
    
    // Integrate addition
    fn add(input: &[u8], output: &mut [u8]) -> bool;
    
    // Scalar multiplication
    fn mul(input: &[u8], output: &mut [u8]) -> bool;
    
    // Pairing
    fn pairing(input: &[u8], output: &mut [u8]) -> bool;
    
    // Verifiy BLS
    fn verify(
        in_vk: VK,
        vk_gammaABC: &[u8],
        in_proof: Proof,
        proof_inputs: &[u8],
    ) -> bool;
}


impl PairingOp for Bls12_381 {}
impl PairingOp for AltBn_128 {}
```

[alt_bn128]: https://github.com/patractlabs/megaclite/blob/master/src/alt_bn128.rs
[bls12_381]: https://github.com/patractlabs/megaclite/blob/master/src/bls12_381.rs
[groth16]: http://www.zeroknowledgeblog.com/index.php/groth16
[ietf]: https://tools.ietf.org/id/draft-yonezawa-pairing-friendly-curves-02.html
