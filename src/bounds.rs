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
            lower: q1,
            upper: q3,
        }
    }

    /// Width of the interquartile range, `upper - lower`.
    pub fn spread(&self) -> f64 {
        self.upper - self.lower
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quartiles_lower_le_upper_and_nonneg_spread() {
        let q = Quartiles::of(&[1.0, 2.0, 3.0, 4.0, 5.0]);
        assert!((q.lower - 2.0).abs() < 1e-9);
        assert!((q.upper - 4.0).abs() < 1e-9);
        assert!(q.lower <= q.upper);
        assert!((q.spread() - 2.0).abs() < 1e-9);
        assert!(q.spread() >= 0.0);
    }

    #[test]
    fn quartiles_even_length_interpolated() {
        let q = Quartiles::of(&[10.0, 20.0, 30.0, 40.0]);
        assert!((q.lower - 17.5).abs() < 1e-9);
        assert!((q.upper - 32.5).abs() < 1e-9);
        assert!((q.spread() - 15.0).abs() < 1e-9);
    }
}
