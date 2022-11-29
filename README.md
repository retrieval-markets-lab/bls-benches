# bls-benches
A set of experiments for testing out methods for aggregate BLS-verification . The main purpose of this repository is to create aset of standardized benchmarks that can be used to inform the [Quarry](https://github.com/retrieval-markets-lab/das-quarry) protocol architecture.

Each sub-folder contains it's own readme with additional details. 


```
bls-benches/
├── snarks/ (experiments using circom ZK-SNARKS)
    └── circuits/ (Circom circuits for bls verification)
    └── scripts/ (scripts for constructing full circuits using Groth16 and PLONK)
    └── tests/ (tests of circuit functions)
└── wasm/ (experiments for BLS verification in wasm)
    └── actors/ (custom Filecoin actors for bls verification)
         └── tests/ (tests of custom actors within FVM)
    └── bls/ (library and benchmarks for running bls in wasm)
        └── benches/ (library benchmarks and tests)
        └── src/ (bls related functions)
   
```

