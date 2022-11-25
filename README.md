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


| n      | native      |  wasm32-wasi | wasm32-wasi (+simd) |
| -----  | ----------- |  ----------- | ----------- |
| 1      | 2.5076 ms   |  7.5714 ms   | _           |
| 10     | 3.5403 ms   |  29.414 ms   | _           |
| 100    | 33.718 ms   |  264.15 ms   | _           |
| 512    | 342.65 ms   |  1684.4 ms   | _           |
| 1000   | 657.69 ms   |  4065.2 ms   | _           |

