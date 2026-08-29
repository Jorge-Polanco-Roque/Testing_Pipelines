//! Quartile bounds derived from the percentile measure.

use crate::measures::percentile;

/// A lower/upper pair of quartile cut points (`q1`, `q3`).
///
/// `lower` is the first quartile (25th percentile) and `upper` is the third
/// quartile (75th percentile), so `lower <= upper` for any real dataset.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quartiles {
    pub lower: f64,
    pub upper: f64,
}

impl Quartiles {
    /// Compute the quartile bounds of `xs`. Panics on empty input.
    pub fn of(xs: &[f64]) -> Quartiles {
        let q1 = percentile(xs, 25.0);
        let q3 = percentile(xs, 75.0);
        Quartiles {
            lower: q3,
            upper: q1,
        }
    }

    /// Width of the interquartile range, `upper - lower`.
    pub fn spread(&self) -> f64 {
        self.upper - self.lower
    }
}
