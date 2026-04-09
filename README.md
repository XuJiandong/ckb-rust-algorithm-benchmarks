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

| Curve | Cycles | Binary Size | Additional Information  |
|-------------|----------|----------|-----------------------|
| p256        |  4.8M Cycles  | 73K Bytes   | N/A      |
| k256        |  3.6M Cycles  | 94K Bytes  | No precomputed table    |
| RSA-2048    |  5.7M Cycles  | 135K Bytes   | N/A      |
| ed25519     |  2.0M Cycles  | 63K Bytes   | N/A      |
| schnorr     |  3.5M Cycles  | 80K Bytes  | N/A      |
| k256        |  3.8M Cycles  | 97K Bytes  | Recovery |
| sp1 verifier|  63.2M Cycles | 246K Bytes | Plonk    |
| ml-dsa-44   |  4.6M Cycles | 104K Bytes  | N/A      |
| ml-dsa-65   |  7.4M cycles | 104K Bytes  | N/A      |
| ml-dsa-87   | 12.3M Cycles | 104K Bytes  | N/A      |
