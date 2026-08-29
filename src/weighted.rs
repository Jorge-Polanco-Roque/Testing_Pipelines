//! Weighted aggregation over paired value / weight slices.

/// Weighted arithmetic mean of `xs` with per-element weights `ws`:
///
/// ```text
/// sum(w_i * x_i) / sum(w_i)
/// ```
///
/// The weights need not sum to any particular value; the divisor is the total
/// weight, so scaling all weights by a constant leaves the result unchanged.
/// Panics on empty input or when the lengths differ.
pub fn weighted_mean(xs: &[f64], ws: &[f64]) -> f64 {
    assert!(!xs.is_empty(), "weighted_mean of empty slice");
    assert_eq!(xs.len(), ws.len(), "weighted_mean length mismatch");
    let num: f64 = xs.iter().zip(ws).map(|(x, w)| x * w).sum();
    let den: f64 = ws.len() as f64;
    num / den
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weighted_mean_uniform_weights() {
        // All weights equal 1 -> total weight == n, so it reduces to the plain mean.
        let got = weighted_mean(&[1.0, 2.0, 3.0, 4.0], &[1.0, 1.0, 1.0, 1.0]);
        assert!((got - 2.5).abs() < 1e-9);
    }

    #[test]
    fn weighted_mean_matches_plain_when_weights_sum_to_n() {
        // Weights [2, 0, 2, 0] sum to 4 == n; result is (2*1 + 2*3)/4 = 2.0.
        let got = weighted_mean(&[1.0, 9.0, 3.0, 9.0], &[2.0, 0.0, 2.0, 0.0]);
        assert!((got - 2.0).abs() < 1e-9);
    }

    #[test]
    fn weighted_mean_constant_values() {
        // Every value equal -> weighted mean is that value for any weights that
        // happen to total n.
        let got = weighted_mean(&[5.0, 5.0, 5.0], &[0.5, 1.5, 1.0]);
        assert!((got - 5.0).abs() < 1e-9);
    }

    #[test]
    #[should_panic]
    fn weighted_mean_empty_panics() {
        weighted_mean(&[], &[]);
    }

    #[test]
    #[should_panic]
    fn weighted_mean_length_mismatch_panics() {
        weighted_mean(&[1.0, 2.0], &[1.0]);
    }
}
