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

doc-upload: doc
	@./scripts/ghp_upload.sh

#
# cortex-m4
#

m4:
	cargo build --target thumbv7em-none-eabi

m4-release:
	cargo build --target thumbv7em-none-eabi --release
