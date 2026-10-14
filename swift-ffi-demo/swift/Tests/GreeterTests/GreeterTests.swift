import Greeter
import Testing

@Test func greets() {
    #expect(greet(name: "Swift") == "Hello, Swift, from Rust!")
}

@Test func summarizes() throws {
    let stats = try summarize(values: [1, 2, 6])
    #expect(stats == Stats(count: 3, mean: 3, min: 1, max: 6))
}

@Test func throwsRustErrors() {
    #expect(throws: StatsError.Empty) { try summarize(values: []) }
    #expect(throws: StatsError.NotFinite) { try summarize(values: [1, .nan]) }
}

@Test func sharesRustObjects() {
    let counter = Counter(start: 41)
    #expect(counter.increment() == 42)
    #expect(counter.value() == 42)
}
