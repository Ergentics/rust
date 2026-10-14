// swift-tools-version: 6.2

import PackageDescription

let package = Package(
    name: "Greeter",
    products: [
        .library(name: "Greeter", targets: ["Greeter"]),
        .executable(name: "greeter-demo", targets: ["greeter-demo"]),
    ],
    targets: [
        // C declarations for the Rust library. build.sh generates the header and stages
        // the Rust static library in .rust-lib/, which is added to the linker search path here.
        .systemLibrary(name: "greeterFFI", path: "Sources/greeterFFI"),
        .target(
            name: "Greeter",
            dependencies: ["greeterFFI"],
            linkerSettings: [.unsafeFlags(["-L\(Context.packageDirectory)/.rust-lib"])]
        ),
        .executableTarget(name: "greeter-demo", dependencies: ["Greeter"]),
        .testTarget(name: "GreeterTests", dependencies: ["Greeter"]),
    ]
)
