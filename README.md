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
make build
```

Run tests:

``` sh
make ci
```

## Benchmark Matrix
Test with [ckb-debugger 0.118](https://github.com/nervosnetwork/ckb-standalone-debugger)

| Curve | Cycles | Binary Size | Additional Information  |
|----------|----------|----------|-----------------------|
| p256     |  3.8M Cycles  | 75K Bytes   | N/A      |
| k256     |  3.7M Cycles  | 107K Bytes  | No precomputed table    |
| RSA-2048 |  5.4M Cycles  | 113K Bytes   | N/A      |
| ed25519  |  2.0M Cycles  | 55K Bytes   | N/A      |
| schnorr  |  3.8M Cycles  | 84K Bytes  | N/A      |
| k256     |  7.8M Cycles  | 122K Bytes  | Recovery |

The k256 recovery can be boosted with this [PR](https://github.com/RustCrypto/signatures/pull/831).
