# Swift FFI demo

A small Rust library called from Swift. It lives in this fork of the Rust compiler as a
self-contained example that the compiler build never uses: `rust/` is its own Cargo workspace.
Outside this directory there is only its workflow and an entry in `REUSE.toml`, so the
repository's license check covers these files.

```
rust/    Rust crate `greeter`, plus the `uniffi-bindgen` tool that generates the Swift bindings
swift/   Swift package `Greeter`: the generated bindings, a demo executable and Swift tests
build.sh builds the Rust crate and generates and stages the bindings for the Swift package
```

## Why UniFFI

The bindings come from [UniFFI](https://mozilla.github.io/uniffi-rs/). It is actively maintained,
ships in production apps on Apple platforms and Linux, and generates idiomatic Swift (structs,
throwing functions, classes) from annotations on ordinary Rust code. The alternative,
swift-bridge, targets only Swift and is maintained more slowly, and the same UniFFI crate could
later generate Kotlin or Python bindings too.

## What crosses the boundary

| Rust                                      | Swift                                   |
| ----------------------------------------- | --------------------------------------- |
| `fn greet(name: String) -> String`        | `func greet(name: String) -> String`    |
| `struct Stats` (`uniffi::Record`)         | `struct Stats: Equatable, Hashable`     |
| `enum StatsError` (`uniffi::Error`)       | `enum StatsError: Error`, thrown        |
| `fn summarize(..) -> Result<Stats, _>`    | `func summarize(values:) throws -> Stats` |
| `struct Counter` (`uniffi::Object`)       | `class Counter`, backed by the Rust heap |

## Build and run

Needs Swift 6.2 or later and Rust 1.91 or later (`rustup`). On Linux, Cargo also needs a C
linker (`cc`).

```sh
./build.sh          # cargo build, then generate swift/Sources/Greeter/greeter.swift and greeterFFI.h
cd swift
swift test
swift run greeter-demo
```

`build.sh` copies the static library into `swift/.rust-lib/` as `libgreeter_rs.a`, which
`Package.swift` adds to the linker search path. The `_rs` suffix matters on macOS: Swift Build
also writes a `libGreeter.a` for the Swift library, and a case-insensitive file system would let
`-lgreeter` pick that file up instead. The generated files are not committed; rerun
`build.sh` after changing the Rust code.

CI for this directory is `.github/workflows/swift-ffi-demo.yml`: Linux with Swift 6.2 and 6.4,
and macOS, which also builds with Swift Build, the default build system from Swift 6.4.
