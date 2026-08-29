//! Range-based measures: minimum, maximum, and their derived statistics.

/// Smallest value in the slice. Panics on empty input.
pub fn min(xs: &[f64]) -> f64 {
    assert!(!xs.is_empty(), "min of empty slice");
    xs.iter().copied().fold(f64::INFINITY, f64::min)
}

/// Largest value in the slice. Panics on empty input.
pub fn max(xs: &[f64]) -> f64 {
    assert!(!xs.is_empty(), "max of empty slice");
    xs.iter().copied().fold(f64::NEG_INFINITY, f64::max)
}

/// Peak-to-peak range of the slice, `max - min`. Panics on empty input.
pub fn peak_to_peak(xs: &[f64]) -> f64 {
    max(xs) - min(xs)
}

/// Midrange of the slice: the midpoint between the smallest and largest
/// values, `(min + max) / 2`. Panics on empty input.
pub fn midrange(xs: &[f64]) -> f64 {
    (min(xs) + max(xs)) / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_max_basic() {
        let xs = [3.0, 1.0, 4.0, 1.0, 5.0, 9.0, 2.0];
        assert!((min(&xs) - 1.0).abs() < 1e-9);
        assert!((max(&xs) - 9.0).abs() < 1e-9);
    }

    #[test]
    fn peak_to_peak_basic() {
        assert!((peak_to_peak(&[2.0, 8.0, 5.0]) - 6.0).abs() < 1e-9);
    }

    #[test]
    fn midrange_from_zero() {
        // Data anchored at zero: midrange is just half the maximum.
        assert!((midrange(&[0.0, 4.0, 2.0]) - 2.0).abs() < 1e-9);
        assert!((midrange(&[0.0, 10.0]) - 5.0).abs() < 1e-9);
    }

    #[test]
    fn midrange_nonzero_min() {
        // (min + max) / 2, not half the range. Fails under the old formula.
        assert!((midrange(&[2.0, 8.0, 5.0, 6.0]) - 5.0).abs() < 1e-9);
        assert!((midrange(&[-4.0, 10.0, 0.0, 7.0]) - 3.0).abs() < 1e-9);
        assert!((midrange(&[100.0, 102.0, 104.0]) - 102.0).abs() < 1e-9);
    }

    #[test]
    #[should_panic]
    fn min_empty_panics() {
        min(&[]);
    }
}
