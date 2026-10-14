#!/usr/bin/env bash
# Builds the Rust library, generates its Swift bindings with UniFFI, and stages both for the
# Swift package in swift/. Run it before `swift build` or `swift test`.
set -euo pipefail
demo=$(cd "$(dirname "$0")" && pwd)
cd "$demo/rust"

cargo build --release --locked

case "$(uname -s)" in
    Darwin) cdylib=libgreeter.dylib ;;
    *) cdylib=libgreeter.so ;;
esac

# uniffi-bindgen reads the interface out of the compiled library. It also runs `cargo metadata`,
# so it has to run from this crate's directory.
out=target/uniffi-swift
rm -rf "$out"
cargo run --release --locked --bin uniffi-bindgen -- \
    generate --library "target/release/$cdylib" --language swift --out-dir "$out"

cp "$out/greeter.swift" "$demo/swift/Sources/Greeter/"
cp "$out/greeterFFI.h" "$demo/swift/Sources/greeterFFI/"
mkdir -p "$demo/swift/.rust-lib"
cp target/release/libgreeter.a "$demo/swift/.rust-lib/"
