
all: ci


ci:
	capsule build --release
	ls -alt target/riscv64imac-unknown-none-elf/release
	ckb-debugger --bin target/riscv64imac-unknown-none-elf/release/schnorr-test	
	ckb-debugger --bin target/riscv64imac-unknown-none-elf/release/p256-test
	ckb-debugger --bin target/riscv64imac-unknown-none-elf/release/k256-test
	ckb-debugger --bin target/riscv64imac-unknown-none-elf/release/rsa-test
	ckb-debugger --bin target/riscv64imac-unknown-none-elf/release/ed25519-test

install:
	cargo install --git https://github.com/nervosnetwork/ckb-standalone-debugger ckb-debugger
	cargo install cross --git https://github.com/cross-rs/cross --rev=6982b6c
	wget 'https://github.com/nervosnetwork/capsule/releases/download/v0.10.4/capsule_v0.10.4_x86_64-linux.tar.gz'
	tar xzvf capsule_v0.10.4_x86_64-linux.tar.gz
	mv capsule_v0.10.4_x86_64-linux/capsule ~/.cargo/bin
