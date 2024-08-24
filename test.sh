#!/bin/bash

set -ex

rustup install nightly

git clone --depth=1 https://github.com/rust-lang/cargo.git

cd cargo

for run in {1..1000}; do
    touch src/bin/cargo/main.rs
    cargo +nightly run
done
