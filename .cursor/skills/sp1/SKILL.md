---
name: sp1
description: How to run and benchmark the SP1 verifier on CKB-VM
---

# SP1 Verifier on CKB-VM

## Build

Build the SP1 verifier from the project root:

```
export CLANG=clang-19
make build CONTRACT=sp1-test
```

## Estimate Cost Cycles

Run from the project root:

```
ckb-debugger --max-cycles 35000000000 --bin build/release/sp1-test
```

An example output of total cycles:
```
All cycles: 6168800817(5883.0M)
```

## Local Debugging

When debugging locally, change the dependency in `Cargo.toml` from:

```toml
sp1-verifier = { git = "https://github.com/XuJiandong/sp1.git", default-features = false, rev="f5586e9" }
```

to a local path:

```toml
sp1-verifier = { path = "../../../sp1/crates/verifier", default-features = false }
```
