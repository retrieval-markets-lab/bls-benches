# BLS Utils 

This library implements utils for testing and benchmarking BLS signing. 


## Benchmarks

To run benchmarks natively: 
```bash
cargo bench verify; cargo bench verify_unsafe
```

To run in wasm with SIMD enabled, run the same steps as above but add the following `RUSTFLAGS` assignment to the build phase: 
```bash
RUSTFLAGS="-C target-feature=+simd128" cargo wasi build --bench=verify --release --features wasi
cp `ls -t target/wasm32-wasi/release/deps/*.wasm | head -n 1` verify-simd.wasm
wasmer run --enable-simd --dir=. verify-simd.wasm -- --bench
```

### Unsafe signing 

To run benchmarks natively: 
```bash
cargo bench verify_unsafe
```

Similary for benchmarking aggregate verification when we use a single message across singers (which is unsafe to rogue-key attacks) run: 
```bash
rustup target add wasm32-wasi
cargo install cargo-wasi
cargo wasi build --bench=verify_unsafe --release
cp `ls -t target/wasm32-wasi/release/deps/*.wasm | head -n 1` verify_unsafe.wasm
wasmer run --dir=. verify_unsafe.wasm -- --bench
```
### Running Benchmarks on WASM modules:
- Ensure that the wasm module is copied or generated to the `/wasm-files` directory. 
- Run `cargo bench --bench wasm_verify features --runtime`. 
- The resulting benchmarks will be testing the runtime of the `verify` function in WASM. 

### WASM Bindings:
 - WASM bindings are generated from the `bindgen-protocol` crate.
 - If the module imports/exports change, then modifications are required to the protocol definition
    for the bindings in `bindgen-protocol`. 

### FVM Gas Metering Injection: 

Gas metering can be injected into a WASM module in the following steps:
 - Ensure that the wasm module is copied or generated to the `/wasm-files` directory. 
 - Change the `file_path` as needed in `main.rs` to run the injection on the correct file. 
 - Run `cargo run --features runtime`

 This will inject the FVM gas metering and run the newly injected module using the bindings
 generated from `fp-bindgen`.



### Current Results 

**MBP M1, 16GB of RAM, average of 100 samples**


| n      | native      |  wasm32-wasi | wasm32-wasi (+simd) |
| -----  | ----------- |  ----------- | ----------- |
| 1      | 1.7318 ms   |  7.6156 ms   | _           |
| 10     | 6.4463 ms   |  29.876 ms   | _           |
| 100    | 56.455 ms   |  268.77 ms   | _           |
| 512    | 345.69 ms   |  1.7006  s   | _           |
| 1000   | 821.04 ms   |  4.1316  s   | _           |


Unsafe (to rogue-key attacks) single message signing:

| n      | native      |  wasm32-wasi | wasm32-wasi (+simd) |
| -----  | ----------- |  ----------- | ----------- |
| 1      | 2.2071 ms   |  9.7771 ms   | _           |
| 10     | 6.9321 ms   |  31.448 ms   | _           |
| 100    | 54.117 ms   |  247.47 ms   | _           |
| 512    | 269.90 ms   |  1.2443  s   | _           |
| 1000   | 524.60 ms   |  2.4170  s   | _           |
