//! Core statistical measures over a slice of `f64`.

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

/// Population variance of the slice: the mean of the squared deviations from
/// the mean, i.e. `sum((x - mean)^2) / n`. Panics on empty input.
pub fn variance(xs: &[f64]) -> f64 {
    assert!(!xs.is_empty(), "variance of empty slice");
    let m = mean(xs);
    let ss: f64 = xs.iter().map(|x| (x - m) * (x - m)).sum();
    ss / xs.len() as f64
}

/// Linear-interpolated percentile of the slice (the `linear` / numpy-default
/// method). `p` is a percentile in `[0.0, 100.0]`. The virtual rank is
/// `r = p/100 * (n - 1)`; the result interpolates linearly between the values
/// at `floor(r)` and `ceil(r)`. `percentile(xs, 50.0)` equals the median.
/// Panics on empty input or when `p` is outside `[0.0, 100.0]`.
pub fn percentile(xs: &[f64], p: f64) -> f64 {
    assert!(!xs.is_empty(), "percentile of empty slice");
    assert!((0.0..=100.0).contains(&p), "percentile p out of range");
    let mut v = xs.to_vec();
    v.sort_by(f64::total_cmp);
    if v.len() == 1 {
        return v[0];
    }
    let rank = p / 100.0 * (v.len() - 1) as f64;
    let lo = rank.floor() as usize;
    let hi = rank.ceil() as usize;
    let frac = rank - rank.floor();
    v[lo] + frac * (v[hi] - v[lo])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_basic() {
        assert!((mean(&[1.0, 2.0, 3.0]) - 2.0).abs() < 1e-9);
    }

    #[test]
    fn variance_constant() {
        // All equal values have zero variance under any correct definition.
        assert!(variance(&[7.0, 7.0, 7.0, 7.0]).abs() < 1e-9);
    }

    #[test]
    fn variance_population() {
        // Population variance = sum((x - mean)^2) / n (divides by n, not n-1).
        assert!((variance(&[1.0, 2.0, 3.0]) - 2.0 / 3.0).abs() < 1e-9);
        assert!((variance(&[2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]) - 4.0).abs() < 1e-9);
    }

    #[test]
    #[should_panic]
    fn variance_empty_panics() {
        variance(&[]);
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

    #[test]
    fn percentile_endpoints() {
        let xs = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert!((percentile(&xs, 0.0) - 1.0).abs() < 1e-9);
        assert!((percentile(&xs, 100.0) - 5.0).abs() < 1e-9);
    }

    #[test]
    fn percentile_exact_index() {
        // n = 5, so ranks land exactly on integer indices for these percentiles.
        let xs = [5.0, 1.0, 4.0, 2.0, 3.0];
        assert!((percentile(&xs, 25.0) - 2.0).abs() < 1e-9);
        assert!((percentile(&xs, 50.0) - 3.0).abs() < 1e-9);
        assert!((percentile(&xs, 75.0) - 4.0).abs() < 1e-9);
    }

    #[test]
    fn percentile_matches_median_odd() {
        let xs = [3.0, 1.0, 2.0];
        assert!((percentile(&xs, 50.0) - median(&xs)).abs() < 1e-9);
    }

    #[test]
    fn percentile_interpolated() {
        // Ranks fall between indices; results must interpolate linearly.
        let xs = [1.0, 2.0, 3.0, 4.0];
        assert!((percentile(&xs, 25.0) - 1.75).abs() < 1e-9);
        assert!((percentile(&xs, 50.0) - 2.5).abs() < 1e-9);
        assert!((percentile(&xs, 75.0) - 3.25).abs() < 1e-9);
        let ys = [10.0, 20.0, 30.0, 40.0, 50.0];
        assert!((percentile(&ys, 90.0) - 46.0).abs() < 1e-9);
    }

    #[test]
    #[should_panic]
    fn percentile_empty_panics() {
        percentile(&[], 50.0);
    }
}
