
all: ci


ci:
	capsule build --release
	ckb-debugger --bin target/riscv64imac-unknown-none-elf/release/p256-test

install:
	wget 'https://github.com/nervosnetwork/ckb-standalone-debugger/releases/download/v0.110.0/ckb-debugger-linux-x64.tar.gz'
	tar zxvf ckb-debugger-linux-x64.tar.gz
	mv ckb-debugger ~/.cargo/bin/ckb-debugger
	cargo install cross --git https://github.com/cross-rs/cross
	wget 'https://github.com/nervosnetwork/capsule/releases/download/v0.10.0/capsule_v0.10.0_x86_64-linux.tar.gz'
	tar xzvf capsule_v0.10.0_x86_64-linux.tar.gz
	mv capsule_v0.10.0_x86_64-linux/capsule ~/.cargo/bin
