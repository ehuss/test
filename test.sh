#!/bin/bash

set -ex

export CARGO_NET_RETRY=0

for package in cargo cargo-make cargo-all-features cargo-deb backtrace deno leptos_icons poem
do
    echo $package
    cargo new a
    cd a
    echo "$package = \"*\"" >> Cargo.toml
    CARGO_HOME=chome cargo generate-lockfile
    cd ..
    rm -rf a
done
