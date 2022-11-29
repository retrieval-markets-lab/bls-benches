# BLS Benches Filecoin actors

We implement 3 custom actors here. 

```
├── actors/ 
    └── agg-multi-verify/ (aggregate signature verification with non-duplicate messages)
    └── agg-single-verify/ (aggregate signature verification with a single shared message)
    └── single-verify/ (single signature verification)
    └── utils/ (testing and benchmarking utilities)
    └── benches/ (actor benchmarks)
   
```


To run benchmarks leveraging the fvm [integration tester](https://github.com/filecoin-project/ref-fvm/tree/master/testing/integration):
```bash
cargo bench
```


## Current Results (time)

Results pending

| n-sign |  single-verify | agg-multi-verify | agg-single-verify |
| -----  | ----------- |  ----------- | ----------- |
| 1      | 1.1789 ms   |  _           | _           |
| 64     | _           |  _           | _           |
| 128    | _           |  _           | _           |
| 256    | _           |  _           | _           |
| 512    | _           |  _           | _           |
| 1024   | _           |  _           | _           |


## Current Results (gas)

| n-sign |  single-verify | agg-multi-verify | agg-single-verify |
| -----  | ----------- |  ----------- | ----------- |
| 1      | _           |  _           | _           |
| 64     | _           |  _           | _           |
| 128    | _           |  _           | _           |
| 256    | _           |  _           | _           |
| 512    | _           |  _           | _           |
| 1024   | _           |  _           | _           |