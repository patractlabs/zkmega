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


| Curve      | Native                           | Time  | Wasm                           | Time     |
|------------|----------------------------------|-------|--------------------------------|----------|
| bls12\_377 | native\_bls12\_377\_add          | 13.34 | wasm\_bls12\_377\_add          | 1239     |
|            | native\_bls12\_377\_mul          | 183.1 | wasm\_bls12\_377\_mul          | 73990    |
|            | native\_bls12\_377\_pairing\_two | 1732  | wasm\_bls12\_377\_pairing\_two | 674300   |
|            | native\_bls12\_377\_pairing\_six | 4005  | wasm\_bls12\_377\_pairing\_six | 1622100  |
|            | native\_bls12\_377\_verify       | 7484  | wasm\_bls12\_377\_verify       | 2701800  |
|------------|----------------------------------|-------|--------------------------------|----------|
| bls12\_381 | native\_bls12\_381\_add          | 13.9  | wasm\_bls12\_381\_add          | 1319     |
|            | native\_bls12\_381\_mul          | 177.1 | wasm\_bls12\_381\_mul          | 71710    |
|            | native\_bls12\_381\_pairing\_two | 1438  | wasm\_bls12\_381\_pairing\_two | 627500   |
|            | native\_bls12\_381\_pairing\_six | 3323  | wasm\_bls12\_381\_pairing\_six | 1492100  |
|            | native\_bls12\_381\_verify       | 6411  | wasm\_bls12\_381\_verify       | 2969400  |
|------------|----------------------------------|-------|--------------------------------|----------|
| bn254      | native\_bn254\_add               | 9.879 | wasm\_bn254\_add               | 723.3    |
|            | native\_bn254\_mul               | 107.7 | wasm\_bn254\_mul               | 35850    |
|            | native\_bn254\_pairing\_two      | 1150  | wasm\_bn254\_pairing\_two      | 317800   |
|            | native\_bn254\_pairing\_six      | 2498  | wasm\_bn254\_pairing\_six      | 755200   |
|            | native\_bn254\_verify            | 4178  | wasm\_bn254\_verify            | 1197100  |
|------------|----------------------------------|-------|--------------------------------|----------|
| bw6\_761   | native\_bw6\_761\_add            | 30.35 | wasm\_bw6\_761\_add            | 4573     |
|            | native\_bw6\_761\_mul            | 963.8 | wasm\_bw6\_761\_mul            | 445600   |
|            | native\_bw6\_761\_pairing\_two   | 5715  | wasm\_bw6\_761\_pairing\_two   | 3044200  |
|            | native\_bw6\_761\_pairing\_six   | 14130 | wasm\_bw6\_761\_pairing\_six   | 7503200  |
|            | native\_bw6\_761\_verify         | 20330 | wasm\_bw6\_761\_verify         | 10803300 |


```bash
# 1. Under the jupiter repo
# 2. Has compiled jupiter-dev
sh ./benchmark.sh
```

[benchmark]: https://github.com/patractlabs/substrate/blob/features/curve-benchmark/bin/node-template/pallets/template/src/lib.rs
[runtime-interface]: https://github.com/patractlabs/substrate/blob/features/curve-benchmark/bin/node-template/io/src/lib.rs
[tests]: https://github.com/patractlabs/megaclite/tree/master/crates/arkworks/src/tests
