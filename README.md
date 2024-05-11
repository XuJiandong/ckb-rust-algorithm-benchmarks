# CKB Rust Algorithm Benchmarks

This project aims to provide benchmarks for the implementation of cryptography
algorithms in Rust on CKB. It now includes:
- p256(secp256r1)
- k256(secp256k1)
- rsa(2048 bits)
- ed25519
- schnorr


## Build and Run

``` sh
capsule build --release
```

Run tests:

``` sh
make ci
```

## Benchmark Matrix
Test with [ckb-debugger 0.115.0-rc2](https://github.com/nervosnetwork/ckb-standalone-debugger)
and [capsule 0.10.4](https://github.com/nervosnetwork/capsule):

| Curve | Cycles | Binary Size | Additional Information  |
|----------|----------|----------|-----------------------|
| p256     |  3.4M Cycles  | 68K Bytes   | N/A      |
| k256     |  2.6M Cycles  | 132K Bytes  | No precomputed table    |
| RSA-2048 |  5.1M Cycles  | 82K Bytes   | N/A      |
| ed25519  |  1.9M Cycles  | 49K Bytes   | N/A      |
| schnorr  |  2.5M Cycles  | 119K Bytes  | N/A      |
