#!/bin/bash

set -ex

rustup install nightly
rustup default nightly

export CARGO_NET_RETRY=0

for package in cargo cargo-make cargo-all-features cargo-deb backtrace deno leptos_icons poem
do
    echo $package
    cargo new a
    cd a
    echo "$package = \"*\"" >> Cargo.toml
    CARGO_LOG=cargo::core::package=trace,cargo::util::network=trace,cargo::sources::registry=trace CARGO_HTTP_DEBUG=true CARGO_HOME=chome cargo generate-lockfile
    cd ..
    rm -rf a
done
