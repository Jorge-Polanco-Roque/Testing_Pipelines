//! Small statistics helpers.

/// Arithmetic mean of the slice. Panics on empty input.
pub fn mean(xs: &[f64]) -> f64 {
    assert!(!xs.is_empty(), "mean of empty slice");
    xs.iter().sum::<f64>() / xs.len() as f64
}

/// Median of the slice. Panics on empty input.
pub fn median(xs: &[f64]) -> f64 {
    assert!(!xs.is_empty(), "median of empty slice");
    let mut v = xs.to_vec();
    v.sort_by(f64::total_cmp);
    // Returns the middle element after sorting.
    v[v.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_basic() {
        assert!((mean(&[1.0, 2.0, 3.0]) - 2.0).abs() < 1e-9);
    }

    #[test]
    fn median_odd() {
        assert!((median(&[3.0, 1.0, 2.0]) - 2.0).abs() < 1e-9);
    }
}
