# BLS Benches

To run benchmarks natively: 
```bash
cargo bench verify
```
To run in wasm install [wasmer](https://wasmer.io/).
Then: 
```bash
rustup target add wasm32-wasi
cargo install cargo-wasi
cargo wasi build --bench=verify --release
cp `ls -t target/wasm32-wasi/release/deps/*.wasm | head -n 1` verify.wasm
wasmer run --dir=. verify.wasm -- --bench
```

To run in wasm with SIMD enabled, run the same steps as above but add the following `RUSTFLAGS` assignment to the build phase: 
```bash
RUSTFLAGS="-C target-feature=+simd128" cargo wasi build --bench=verify --release
cp `ls -t target/wasm32-wasi/release/deps/*.wasm | head -n 1` verify-simd.wasm
wasmer run --enable-simd --dir=. verify-simd.wasm -- --bench
```

## Current Results 

**MBP M1, 16GB of RAM, average of 100 samples**


| n      | native      |  wasm32-wasi | wasm-wasi (+simd) |
| -----  | ----------- |  ----------- | ----------- |
| 1      | 2.5076 ms   |  11.964 ms   | _           |
| 10     | 3.5403 ms   |  47.724 ms   | _           |
| 100    | 33.718 ms   |  418.48 ms   | _           |
| 1000   | 657.69 ms   |  6456.2 ms   | _           |

