# Benchmark

We have constructed [the tests of megaclite curves][tests] in [pallet-template][benchmark] which imports the curves from megaclite directly(*Wasm*) and with `runtime-interface`(*Native*).


## Building

```bash
# Clone the branch `curve-benchmark` of our fork
git clone https://github.com/patractlabs/substrate.git \
    --branch features/curve-benchmark \
    --depth =1

# Build the template
cargo build -p node-template --all-features --release

# Check the command benchmark works fine
# ./target/release/node-teamplate benchmark -p template -e wasm_bls_12_381_add
./target/release/node-teamplate benchmark -p template -e native_bls_12_381_add

```

## Result

> Extrinsics listed below are ready to test

| Curve      | Native                           | Time | Wasm                           | Time |
|------------|----------------------------------|------|--------------------------------|------|
| bls12\_377 | native\_bls12\_377\_add          |      | wasm\_bls12\_377\_add          |      |
|            | native\_bls12\_377\_mul          |      | wasm\_bls12\_377\_mul          |      |
|            | native\_bls12\_377\_pairing\_two |      | wasm\_bls12\_377\_pairing\_two |      |
|            | native\_bls12\_377\_pairing\_six |      | wasm\_bls12\_377\_pairing\_six |      |
|------------|----------------------------------|------|--------------------------------|------|
| bls12\_381 | native\_bls12\_381\_add          |      | wasm\_bls12\_381\_add          |      |
|            | native\_bls12\_381\_mul          |      | wasm\_bls12\_381\_mul          |      |
|            | native\_bls12\_381\_pairing\_two |      | wasm\_bls12\_381\_pairing\_two |      |
|            | native\_bls12\_381\_pairing\_six |      | wasm\_bls12\_381\_pairing\_six |      |
|------------|----------------------------------|------|--------------------------------|------|
| bn254      | native\_bn254\_add               |      | wasm\_bn254\_add               |      |
|            | native\_bn254\_mul               |      | wasm\_bn254\_mul               |      |
|            | native\_bn254\_pairing\_two      |      | wasm\_bn254\_pairing\_two      |      |
|            | native\_bn254\_pairing\_six      |      | wasm\_bn254\_pairing\_six      |      |
|------------|----------------------------------|------|--------------------------------|------|
| bw6\_761   | native\_bw6\_761\_add            |      | wasm\_bw6\_761\_add            |      |
|            | native\_bw6\_761\_mul            |      | wasm\_bw6\_761\_mul            |      |
|            | native\_bw6\_761\_pairing\_two   |      | wasm\_bw6\_761\_pairing\_two   |      |
|            | native\_bw6\_761\_pairing\_six   |      | wasm\_bw6\_761\_pairing\_six   |      |
| cp6\_782   | native\_cp6\_782\_add            |      | wasm\_cp6\_782\_add            |      |
|            | native\_cp6\_782\_mul            |      | wasm\_cp6\_782\_mul            |      |
|            | native\_cp6\_782\_pairing\_two   |      | wasm\_cp6\_782\_pairing\_two   |      |
|            | native\_cp6\_782\_pairing\_six   |      | wasm\_cp6\_782\_pairing\_six   |      |

```bash
# 1. Under the jupiter repo
# 2. Has compiled node-template
sh ./benchmark.sh
```
