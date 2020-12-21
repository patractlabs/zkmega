# Example

## Call curves in ink!

| curve      | add        | mul        | pairing    |
|------------|------------|------------|------------|
| bls12\_377 | 0x01000000 | 0x01000001 | 0x01000002 |
| bls12\_381 | 0x01000010 | 0x01000011 | 0x01000012 |
| bn254      | 0x01000020 | 0x01000021 | 0x01000022 |
| bw6\_761   | 0x01000030 | 0x01000031 | 0x01000032 |

```rust
let result = ink_env::call_chain_extension(func_id, &Vec::from(input))?
```


## MIMC

```rust
use zkp_u256::{U256, Zero};
use merkle_tree::mimc::{mimc,mimc_with_key}
let message = U256::from_decimal_str("49").unwrap();
let in_key = U256::zero();
assert_eq!(
    mimc(b"1"),
    mimc_with_key(vec![&message], &in_key)
);
```


## Merkle Tree

```rust
use merkle_tree::MerkleTree;
let mut mt = MerkleTree::default();
let message = b"49";
let (leaf, index) = mt.insert(message).unwrap();
assert_eq!(mt.update(), mt.get_root());
let merkle_proof = mt.get_proof(index);
assert!(mt.verify_merkle_proof(leaf, merkle_proof, index));

let message = b"50";
let (leaf, index) = mt.insert(message).unwrap();
assert_eq!(mt.update(), mt.get_root());
let merkle_proof = mt.get_proof(index);
assert!(mt.verify_merkle_proof(leaf, merkle_proof, index));

let message = b"51";
let (leaf, index) = mt.insert(message).unwrap();
assert_eq!(mt.update(), mt.get_root());
let merkle_proof = mt.get_proof(index);
assert!(mt.verify_merkle_proof(leaf, merkle_proof, index));
```

## EDDSA

> TODO
