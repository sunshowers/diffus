#!/usr/bin/env sh
cargo test
cargo test --all-features

cargo fmt -- --check
