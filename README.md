# CKB Rust Algorithm Benchmarks

This project aims to provide benchmarks for the implementation of cryptography
algorithms in Rust on CKB. It now includes:
- p256(secp256r1)
- k256(secp256k1)
- rsa(2048 bits)
- ed25519
- schnorr
- sp1 verifier(zkvm)
- ml-dsa

## Build and Run

``` sh
make build
```

Run tests:

``` sh
make ci
```

## Benchmark Matrix
Test with [ckb-debugger 1.1.0](https://github.com/nervosnetwork/ckb-standalone-debugger)

| Algorithm | Cycles | Binary Size | Additional Information  |
|-------------|----------|----------|-----------------------|
| p256        |  6.4M Cycles  |  95K Bytes  | N/A                    |
| k256        |  3.8M Cycles  | 116K Bytes  | No precomputed table   |
| RSA-2048    |  6.5M Cycles  | 158K Bytes  | N/A                    |
| ed25519     |  1.9M Cycles  |  88K Bytes  | N/A                    |
| schnorr     |  3.7M Cycles  | 102K Bytes  | N/A                    |
| k256        |  4.0M Cycles  | 118K Bytes  | Recovery               |
| sp1 verifier | 66.2M Cycles  | 262K Bytes  | Plonk                  |
| ml-dsa-44   |  3.3M Cycles  | 137K Bytes  | N/A                    |
| ml-dsa-65   |  5.0M Cycles  | 137K Bytes  | N/A                    |
| ml-dsa-87   |  7.6M Cycles  | 137K Bytes  | N/A                    |
