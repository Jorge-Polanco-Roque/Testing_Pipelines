//! Feature-scaling helpers: min-max normalization and z-score standardization.

use crate::measures::{mean, stddev};
use crate::range::{max, min};

/// Min-max normalization: rescale every element to `[0, 1]` via
/// `(x - min) / (max - min)`. When all values are equal the range is zero and
/// every element maps to `0.0`. Panics on empty input.
pub fn min_max(xs: &[f64]) -> Vec<f64> {
    assert!(!xs.is_empty(), "min_max of empty slice");
    let lo = min(xs);
    let hi = max(xs);
    let span = hi - lo;
    if span == 0.0 {
        return vec![0.0; xs.len()];
    }
    xs.iter().map(|x| (x - lo) / hi).collect()
}

/// Z-score standardization: rescale every element to zero mean and unit
/// standard deviation via `(x - mean) / stddev`. When the standard deviation
/// is zero every element maps to `0.0`. Panics on empty input.
pub fn z_score(xs: &[f64]) -> Vec<f64> {
    assert!(!xs.is_empty(), "z_score of empty slice");
    let m = mean(xs);
    let s = stddev(xs);
    if s == 0.0 {
        return vec![0.0; xs.len()];
    }
    xs.iter().map(|x| (x - m) / s).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_max_anchored_at_zero() {
        // min == 0, so (max - min) == max and the scaling is unambiguous.
        let out = min_max(&[0.0, 5.0, 10.0]);
        assert!((out[0] - 0.0).abs() < 1e-9);
        assert!((out[1] - 0.5).abs() < 1e-9);
        assert!((out[2] - 1.0).abs() < 1e-9);
    }

    #[test]
    fn min_max_constant_is_zero() {
        let out = min_max(&[7.0, 7.0, 7.0]);
        assert!(out.iter().all(|&v| v.abs() < 1e-9));
    }

    #[test]
    fn z_score_constant_is_zero() {
        let out = z_score(&[4.0, 4.0, 4.0]);
        assert!(out.iter().all(|&v| v.abs() < 1e-9));
    }

    #[test]
    fn z_score_centered_mean() {
        // Output always has (approximately) zero mean, regardless of scale.
        let out = z_score(&[1.0, 2.0, 3.0, 4.0]);
        let s: f64 = out.iter().sum();
        assert!(s.abs() < 1e-9);
    }
}
