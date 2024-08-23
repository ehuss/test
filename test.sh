#!/bin/bash

set -ex

cd foo

for run in {1..1000}; do
    touch src/main.rs
    cargo run
done
