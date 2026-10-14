//! A tiny Rust library exposed to Swift through UniFFI.
//!
//! Everything marked `#[uniffi::export]` or deriving a `uniffi` trait shows up in the generated
//! Swift bindings: free functions, records (value types), errors and objects (reference types).

use std::fmt;
use std::sync::Mutex;

uniffi::setup_scaffolding!();

/// Returns a greeting built in Rust.
#[uniffi::export]
pub fn greet(name: String) -> String {
    format!("Hello, {name}, from Rust!")
}

/// Summary statistics, passed to Swift as a struct.
#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct Stats {
    pub count: u64,
    pub mean: f64,
    pub min: f64,
    pub max: f64,
}

/// Errors surface in Swift as a thrown `enum`.
#[derive(Debug, PartialEq, uniffi::Error)]
pub enum StatsError {
    Empty,
    NotFinite,
}

impl fmt::Display for StatsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatsError::Empty => f.write_str("no values given"),
            StatsError::NotFinite => f.write_str("values must be finite"),
        }
    }
}

impl std::error::Error for StatsError {}

/// Computes count, mean, min and max of `values`.
#[uniffi::export]
pub fn summarize(values: Vec<f64>) -> Result<Stats, StatsError> {
    if values.is_empty() {
        return Err(StatsError::Empty);
    }
    if values.iter().any(|v| !v.is_finite()) {
        return Err(StatsError::NotFinite);
    }
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    Ok(Stats { count: values.len() as u64, mean, min, max })
}

/// A counter that lives on the Rust heap; Swift holds a reference to it.
#[derive(Debug, uniffi::Object)]
pub struct Counter {
    value: Mutex<u64>,
}

#[uniffi::export]
impl Counter {
    #[uniffi::constructor]
    pub fn new(start: u64) -> Self {
        Counter { value: Mutex::new(start) }
    }

    /// Adds one and returns the new value.
    pub fn increment(&self) -> u64 {
        let mut value = self.value.lock().unwrap();
        *value += 1;
        *value
    }

    pub fn value(&self) -> u64 {
        *self.value.lock().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets() {
        assert_eq!(greet("Swift".into()), "Hello, Swift, from Rust!");
    }

    #[test]
    fn summarizes() {
        let stats = summarize(vec![1.0, 2.0, 6.0]).unwrap();
        assert_eq!(stats, Stats { count: 3, mean: 3.0, min: 1.0, max: 6.0 });
    }

    #[test]
    fn rejects_bad_input() {
        assert_eq!(summarize(vec![]), Err(StatsError::Empty));
        assert_eq!(summarize(vec![1.0, f64::NAN]), Err(StatsError::NotFinite));
    }

    #[test]
    fn counts() {
        let counter = Counter::new(41);
        assert_eq!(counter.increment(), 42);
        assert_eq!(counter.value(), 42);
    }
}
