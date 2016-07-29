PRGDEFAULT=thumbv7em-none-eabi

if [ -z "$PRGTARGET" ]; then
    export PRGTARGET=$PRGDEFAULT
fi

if [ "$PRGTARGET" == "test" ] ; then
    cargo test --verbose
else
    export RUST_TARGET_PATH=$RUST_TARGET_PATH:./build_targets
    cargo build --verbose --target $PRGTARGET
fi
