# p256-test

Build contracts:

``` sh
capsule build --release
```

Run tests:

``` sh
ckb-debugger --bin target/riscv64imac-unknown-none-elf/release/p256-test
ckb-debugger --bin target/riscv64imac-unknown-none-elf/release/k256-test
```


## Benchmark Matrix
Test with [ckb-debugger 0.111-rc8](https://github.com/nervosnetwork/ckb-standalone-debugger/commit/75e81f9490d6186ad8d9accbf39040640fcac228)
and [capsule 0.10.0](https://github.com/nervosnetwork/capsule):

| Curve | Cycles | Binary Size | Additional Information |
|----------|----------|----------|-----------------------|
| p256     |  4.18 M Cycles  | 53 K Bytes  | N/A      |
| k256     |  2.87 M Cycles  | 84 K Bytes  | No precomputed table    |
| RSA-2048 |  5.23 M Cycles  | 109 K Bytes | N/A      |
| ed25519  |  1.92 M Cycles  | 70 K Bytes  | N/A      |
