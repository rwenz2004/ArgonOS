.PHONY: build

build:
	cargo build --release
	rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/kernel -O binary kernel.bin
	