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
    let mid = v.len() / 2;
    if v.len() % 2 == 1 {
        v[mid]
    } else {
        // even length: average of the two middle elements
        (v[mid - 1] + v[mid]) / 2.0
    }
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

    #[test]
    fn median_even() {
        assert!((median(&[1.0, 2.0, 3.0, 4.0]) - 2.5).abs() < 1e-9);
        assert!((median(&[10.0, 2.0, 8.0, 4.0]) - 6.0).abs() < 1e-9);
    }
}
