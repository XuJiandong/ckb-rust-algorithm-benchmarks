
all: ci


ci:
	capsule build --release
	ls -alht target/riscv64imac-unknown-none-elf/release
	ckb-debugger --bin target/riscv64imac-unknown-none-elf/release/k256-recovery-test
	ckb-debugger --bin target/riscv64imac-unknown-none-elf/release/schnorr-test	
	ckb-debugger --bin target/riscv64imac-unknown-none-elf/release/p256-test
	ckb-debugger --bin target/riscv64imac-unknown-none-elf/release/k256-test
	ckb-debugger --bin target/riscv64imac-unknown-none-elf/release/rsa-test
	ckb-debugger --bin target/riscv64imac-unknown-none-elf/release/ed25519-test

install:
	cargo install --git https://github.com/nervosnetwork/ckb-standalone-debugger ckb-debugger --tag 0.115.0-rc2
	cargo install cross --git https://github.com/cross-rs/cross --rev=6982b6c
	cargo install ckb-capsule --git https://github.com/nervosnetwork/capsule.git --tag v0.10.4
