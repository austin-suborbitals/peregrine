PRGTARGET=thumbv7em-none-eabi

RUST_TARGET_PATH=$RUST_TARGET_PATH:./build_targets cargo build --verbose --target $PRGTARGET
