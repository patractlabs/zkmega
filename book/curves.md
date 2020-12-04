# Curves

|             | bls12\_377 | bls12\_381 | bn254 | bw6\_761 | cp6\_782 |
|-------------|------------|------------|-------|----------|----------|
| arkworks    | x          | x          | x     | x        | x        |
| matter-labs |            | x          | x     |          |          |
| zcash       |            |            | x     |          |          |

We implement megaclite curves based on [zcash][zcash], [matter-labs][matter-labs] 
and [arkworks][arkworks], all of the curves megaclite implements contain `add`, `mul` 
and `pairing`, including `verify`.


[zcash]: https://github.com/patractlabs/megaclite/tree/master/crates/zcash
[matter-labs]: https://github.com/patractlabs/megaclite/tree/master/crates/matter-labs
[arkworks]: https://github.com/patractlabs/megaclite/tree/master/crates/arkworks
