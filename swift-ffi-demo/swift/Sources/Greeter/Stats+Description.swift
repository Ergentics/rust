// The rest of this module is greeter.swift, which uniffi-bindgen generates from the Rust crate.

extension Stats: CustomStringConvertible {
    public var description: String {
        "\(count) values, mean \(mean), range \(min)...\(max)"
    }
}
