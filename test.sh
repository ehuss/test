#!/bin/bash

set -ex

rustup install 1.71.1
rustup default 1.71.1
cargo -Vv

export CARGO_NET_RETRY=0
export CARGO_HTTP_MULTIPLEXING=false

for package in cargo cargo-make cargo-all-features cargo-deb backtrace deno leptos_icons poem
do
    echo $package
    cargo new a
    cd a
    echo "$package = \"*\"" >> Cargo.toml
    CARGO_LOG=network=trace,cargo::ops::registry=trace,cargo::core::package=trace,cargo::util::network=trace,cargo::sources::registry=trace \
        CARGO_HTTP_DEBUG=true \
        CARGO_HOME=chome \
        cargo generate-lockfile
    cd ..
    rm -rf a
done
