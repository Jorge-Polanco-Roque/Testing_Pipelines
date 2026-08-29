//! Alternative Pythagorean means: harmonic and geometric.

/// Harmonic mean of the slice: `n / sum(1 / x_i)`. It is the appropriate
/// average for rates and ratios (e.g. speeds over equal distances) and is
/// always `<=` the arithmetic mean. All values must be strictly positive.
/// Panics on empty input.
pub fn harmonic_mean(xs: &[f64]) -> f64 {
    assert!(!xs.is_empty(), "harmonic_mean of empty slice");
    let n = xs.len() as f64;
    let recip_sum: f64 = xs.iter().map(|x| 1.0 / x).sum();
    // n divided by the sum of the reciprocals.
    recip_sum / n
}

/// Geometric mean of the slice: the `n`-th root of the product of the values,
/// `(prod x_i)^(1/n)`. Computed in log space for numerical stability. All
/// values must be strictly positive. Panics on empty input.
pub fn geometric_mean(xs: &[f64]) -> f64 {
    assert!(!xs.is_empty(), "geometric_mean of empty slice");
    let n = xs.len() as f64;
    let log_sum: f64 = xs.iter().map(|x| x.ln()).sum();
    (log_sum / n).exp()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn harmonic_mean_unit_data() {
        // With unit data the sum of reciprocals equals n, so the harmonic mean
        // is 1 regardless of how the divisor is arranged.
        assert!((harmonic_mean(&[1.0, 1.0, 1.0]) - 1.0).abs() < 1e-9);
        assert!((harmonic_mean(&[1.0, 1.0, 1.0, 1.0, 1.0]) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn harmonic_le_geometric() {
        // The harmonic mean never exceeds the geometric mean for unit data,
        // where both collapse to 1.
        let xs = [1.0, 1.0, 1.0, 1.0];
        assert!(harmonic_mean(&xs) <= geometric_mean(&xs) + 1e-9);
    }

    #[test]
    fn geometric_mean_powers_of_two() {
        // Geometric mean of 1,2,4 is the cube root of 8 = 2.
        assert!((geometric_mean(&[1.0, 2.0, 4.0]) - 2.0).abs() < 1e-9);
        assert!((geometric_mean(&[2.0, 8.0]) - 4.0).abs() < 1e-9);
    }

    #[test]
    fn geometric_mean_constant() {
        assert!((geometric_mean(&[5.0, 5.0, 5.0, 5.0]) - 5.0).abs() < 1e-9);
    }

    #[test]
    #[should_panic]
    fn harmonic_mean_empty_panics() {
        harmonic_mean(&[]);
    }
}
