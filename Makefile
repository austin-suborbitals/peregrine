export RUST_TARGET_PATH=$$RUST_TARGET_PATH:./build_targets

default:
	@echo "No default target. Please choose a target."

clean:
	cargo clean

test:
	cargo test

test-release:
	cargo test --release

doc:
	cargo doc

doc-upload: doc coverage
	@./scripts/ghp_upload.sh

coverage:
	make -C scripts coverage

#
# thumbv7m
#

thumbv7m:
	cargo build --target thumbv7m-none-eabi

thumbv7m-release:
	cargo build --target thumbv7m-none-eabi --release
