clippy:
	cargo clippy --all-targets

check-lint: clippy
	cargo fmt -- --check

lint: clippy
	cargo fmt

build:
	cargo build

test_zk_program:
	cargo test

clean_contracts:
	forge clean

clean_zkvm:
	cargo clean

clean:
	make clean_contracts; make test_zk_program
test_solidity_dev: build
	 RISC0_DEV_MODE=true forge test -vvvvv --match-contract OrderedListTest

test_solidity_normal: build
	RISC0_DEV_MODE=false forge test -vvvvv --match-contract OrderedListTest
