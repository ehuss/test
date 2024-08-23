#!/bin/bash

set -ex

rustup install nightly

cd foo

for run in {1..1000}; do
    touch src/main.rs
    cargo +nightly run
done
