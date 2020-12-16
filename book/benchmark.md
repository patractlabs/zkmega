# Benchmark

We have constructed [the tests of megaclite curves][tests] in [pallet-template][benchmark] which imports the 
curves from megaclite directly(*Wasm*) and with [runtime-interface][runtime-interface](*Native*).


## Building

```bash
# Clone the branch `curve-benchmark` of our fork
git clone https://github.com/patractlabs/jupiter.git \
    --branch features/runtime-interfaces \
    --depth =1

# Build the template
cargo build -p jupiter-dev --all-features --release

# Check the command benchmark works fine
# ./target/release/jupiter-dev benchmark -p pallet_template -e wasm_bls_12_381_add
./target/release/jupiter-dev benchmark -p pallet_template -e wasm_bls_12_381_add

```

## Result


| memory              | processor                           |
|---------------------|-------------------------------------|
| 64GiB System memory | AMD Ryzen 9 5900X 12-Core Processor |

Here we test the curevs on ubuntu LTS 20.04


| Curve      | Native                           | Time  | Wasm                           | Time   |
|------------|----------------------------------|-------|--------------------------------|--------|
| bls12\_377 | native\_bls12\_377\_add          | 9.588 | wasm\_bls12\_377\_add          | 29.02  |
|            | native\_bls12\_377\_mul          | 183.1 | wasm\_bls12\_377\_mul          | 1893   |
|            | native\_bls12\_377\_pairing\_two | 1732  | wasm\_bls12\_377\_pairing\_two | 15310  |
|            | native\_bls12\_377\_pairing\_six | 4005  | wasm\_bls12\_377\_pairing\_six | 36460  |
|            | native\_bls12\_377\_verify       | 7484  | wasm\_bls12\_377\_verify       | 64680  |
|------------|----------------------------------|-------|--------------------------------|--------|
| bls12\_381 | native\_bls12\_381\_add          | 13.9  | wasm\_bls12\_381\_add          | 28.31  |
|            | native\_bls12\_381\_mul          | 177.1 | wasm\_bls12\_381\_mul          | 1879   |
|            | native\_bls12\_381\_pairing\_two | 1438  | wasm\_bls12\_381\_pairing\_two | 14770  |
|            | native\_bls12\_381\_pairing\_six | 3323  | wasm\_bls12\_381\_pairing\_six | 34450  |
|            | native\_bls12\_381\_verify       | 6411  | wasm\_bls12\_381\_verify       | 63260  |
|------------|----------------------------------|-------|--------------------------------|--------|
| bn254      | native\_bn254\_add               | 5.631 | wasm\_bn254\_add               | 16.05  |
|            | native\_bn254\_mul               | 107.7 | wasm\_bn254\_mul               | 534.3  |
|            | native\_bn254\_pairing\_two      | 1150  | wasm\_bn254\_pairing\_two      | 5061   |
|            | native\_bn254\_pairing\_six      | 2498  | wasm\_bn254\_pairing\_six      | 12180  |
|            | native\_bn254\_verify            | 4178  | wasm\_bn254\_verify            | 19850  |
|------------|----------------------------------|-------|--------------------------------|--------|
| bw6\_761   | native\_bw6\_761\_add            | 30.35 | wasm\_bw6\_761\_add            | 26.79  |
|            | native\_bw6\_761\_mul            | 963.8 | wasm\_bw6\_761\_mul            | 14630  |
|            | native\_bw6\_761\_pairing\_two   | 5715  | wasm\_bw6\_761\_pairing\_two   | 60960  |
|            | native\_bw6\_761\_pairing\_six   | 14130 | wasm\_bw6\_761\_pairing\_six   | 158600 |
|            | native\_bw6\_761\_verify         | 20330 | wasm\_bw6\_761\_verify         | 299800 |


```bash
# 1. Under the jupiter repo
# 2. Has compiled jupiter-dev
sh ./benchmark.sh
```

[benchmark]: https://github.com/patractlabs/substrate/blob/features/curve-benchmark/bin/node-template/pallets/template/src/lib.rs
[runtime-interface]: https://github.com/patractlabs/substrate/blob/features/curve-benchmark/bin/node-template/io/src/lib.rs
[tests]: https://github.com/patractlabs/megaclite/tree/master/crates/arkworks/src/tests
