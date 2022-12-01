# BLS Benches Filecoin actors

We implement 3 custom actors here. 

```
├── contracts/ 
    └── agg-multi-verify/ (aggregate signature verification with non-duplicate messages)
    └── agg-single-verify/ (aggregate signature verification with a single shared message)
    └── single-verify/ (single signature verification using FVM syscall)
   
```


To run benchmarks leveraging the fvm [integration tester](https://github.com/filecoin-project/ref-fvm/tree/master/testing/integration):
```bash
cargo bench
```

## Current Results

To benchmark we: 

- average over 10 calls to a contract
- use the same deployed contract over all 10 calls 
- use a p-value significance level of 0.10
- message size (in bytes) is `num_signatures x 64` 

### time ⏳

Machine: M1 Ultra - 64GB RAM

| n-sign |  single-verify | agg-multi-verify | agg-single-verify |
| -----  | ----------- |  ----------- | ----------- |
| 1      | 1.1789 ms   |  211.12 ms   | 211.85 ms   |
| 64     | _           |  6.0021 s    | 3.4756 s    |
| 128    | _           |  11.954 s    | 6.7789 s    |
| 256    | _           |  24.394 s    | 13.130 s    |
| 512    | _           |  51.613 s    | 26.141 s    |
| 1024   | _           |  117.06 s    | 52.347 s    |


### gas used (**units**: $\times 10^{10}$) ⛽️


| n-sign |  single-verify | agg-multi-verify | agg-single-verify |
| -----  | ----------- |  ----------- | ----------- |
| 1      | 0.0017      |  0.1436      | 0.1436      |
| 64     | _           |  4.1705      | 2.3758      |
| 128    | _           |  8.4177      | 4.6435      |
| 256    | _           |  17.385      | 9.1788      |
| 512    | _           |  37.214      | 18.249      |
| 1024   | _           |  84.444      | 36.390      |
