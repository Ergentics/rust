import Greeter

print(greet(name: "Swift"))

let stats = try summarize(values: [3, 1, 4, 1, 5, 9, 2, 6])
print("Stats from Rust: \(stats)")

do {
    _ = try summarize(values: [])
} catch {
    print("Rust error caught in Swift: \(error)")
}

let counter = Counter(start: 41)
print("Rust counter: \(counter.increment())")
