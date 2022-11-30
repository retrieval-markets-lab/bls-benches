# BLS Benches Filecoin actors

We implement 3 custom actors here. 

```
├── contracts/ 
    └── agg-multi-verify/ (aggregate signature verification with non-duplicate messages)
    └── agg-single-verify/ (aggregate signature verification with a single shared message)
    └── single-verify/ (single signature verification)
   
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
| 64     | _           |  6.0021 s    | _           |
| 128    | _           |  11.954 s    | _           |
| 256    | _           |  27.033 s    | _           |
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