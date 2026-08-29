//! Aggregated five-number-style summary of a dataset.

use crate::bounds::Quartiles;
use crate::measures::{mean, median};

/// A compact statistical summary aggregating several measures of a dataset.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Summary {
    pub mean: f64,
    pub median: f64,
    /// First quartile (25th percentile).
    pub q1: f64,
    /// Third quartile (75th percentile).
    pub q3: f64,
    /// Interquartile range, `q3 - q1`.
    pub iqr: f64,
}

impl Summary {
    /// Build a summary of `xs`. Panics on empty input.
    pub fn of(xs: &[f64]) -> Summary {
        let q = Quartiles::of(xs);
        Summary {
            mean: mean(xs),
            median: median(xs),
            q1: q.lower,
            q3: q.upper,
            iqr: q.spread(),
        }
    }

    /// One-line human-readable rendering of the summary.
    pub fn format(&self) -> String {
        format!(
            "mean={:.4} median={:.4} q1={:.4} q3={:.4} iqr={:.4}",
            self.mean, self.median, self.q1, self.q3, self.iqr
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summary_mean_median() {
        let s = Summary::of(&[1.0, 2.0, 3.0, 4.0, 5.0]);
        assert!((s.mean - 3.0).abs() < 1e-9);
        assert!((s.median - 3.0).abs() < 1e-9);
    }

    #[test]
    fn summary_quartiles_ordered_nonneg_iqr() {
        let s = Summary::of(&[1.0, 2.0, 3.0, 4.0, 5.0]);
        assert!((s.q1 - 2.0).abs() < 1e-9);
        assert!((s.q3 - 4.0).abs() < 1e-9);
        assert!((s.iqr - 2.0).abs() < 1e-9);
        assert!(s.q1 <= s.q3 && s.iqr >= 0.0);
    }

    #[test]
    fn summary_format_shape() {
        let s = Summary::of(&[1.0, 2.0, 3.0, 4.0, 5.0]);
        let out = s.format();
        assert!(out.contains("mean="));
        assert!(out.contains("iqr="));
    }
}
