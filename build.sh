#!/bin/bash
# Bunch of flags to minimize the build size. 
# Thank you https://github.com/johnthagen/min-sized-rust
RUSTFLAGS="-Zlocation-detail=none -Zunstable-options -Cpanic=immediate-abort" cargo +nightly build \
    -Z build-std=std,panic_abort \
    -Z build-std-features="optimize_for_size" \
    --target x86_64-unknown-linux-gnu --release
