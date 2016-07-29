if [ -z "$PRGTARGET" ]; then
    export PRGTARGET=thumbv7em-none-eabi
fi

if [ "$PRGTARGET" == "host" ] ; then
    cargo test --verbose
else
    export RUST_TARGET_PATH=$RUST_TARGET_PATH:./build_targets
    cargo build --verbose --target $PRGTARGET
fi
