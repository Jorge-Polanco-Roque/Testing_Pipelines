//! Higher-order moments: the population (Fisher–Pearson) skewness.

use crate::measures::{mean, stddev};

/// Population skewness (the Fisher–Pearson coefficient) of the slice:
///
/// ```text
/// g1 = ( (1/n) * sum((x - mean)^3) ) / stddev^3
/// ```
///
/// where `stddev` is the population standard deviation (divisor `n`). It is
/// zero for any symmetric distribution, positive when the right tail is longer
/// and negative when the left tail is longer. Returns `0.0` when the standard
/// deviation is zero (constant data). Panics on empty input.
pub fn skewness(xs: &[f64]) -> f64 {
    assert!(!xs.is_empty(), "skewness of empty slice");
    let m = mean(xs);
    let s = stddev(xs);
    if s == 0.0 {
        return 0.0;
    }
    // Third central moment: the *mean* of the cubed deviations (divisor n).
    let n = xs.len() as f64;
    let m3: f64 = xs.iter().map(|x| (x - m).powi(3)).sum::<f64>() / n;
    m3 / s.powi(3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skewness_symmetric_is_zero() {
        // Symmetric data: sum of cubed deviations is exactly 0, so the missing
        // 1/n factor cannot change the result.
        assert!(skewness(&[1.0, 2.0, 3.0, 4.0, 5.0]).abs() < 1e-9);
        assert!(skewness(&[-2.0, -1.0, 0.0, 1.0, 2.0]).abs() < 1e-9);
    }

    #[test]
    fn skewness_constant_is_zero() {
        assert!(skewness(&[7.0, 7.0, 7.0]).abs() < 1e-9);
    }

    #[test]
    fn skewness_asymmetric_magnitude() {
        // Regression: without the `/ n` on the third central moment these are
        // inflated by a factor of n.
        assert!((skewness(&[0.0, 0.0, 0.0, 0.0, 10.0]) - 1.5).abs() < 1e-9);
        assert!((skewness(&[2.0, 2.0, 2.0, 8.0]) - 1.1547005383792515).abs() < 1e-9);
        assert!((skewness(&[1.0, 1.0, 1.0, 1.0, 1.0, 7.0]) - 1.7888543819998317).abs() < 1e-9);
    }

    #[test]
    #[should_panic]
    fn skewness_empty_panics() {
        skewness(&[]);
    }
}
