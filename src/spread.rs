//! Robust spread measures: interquartile range and the trimmed mean.

/// Linear-interpolated quantile helper (numpy-default method) over an
/// already-sorted slice. `q` is in `[0.0, 1.0]`.
fn quantile_sorted(v: &[f64], q: f64) -> f64 {
    if v.len() == 1 {
        return v[0];
    }
    let rank = q * (v.len() - 1) as f64;
    let lo = rank.floor() as usize;
    let hi = rank.ceil() as usize;
    let frac = rank - rank.floor();
    v[lo] + frac * (v[hi] - v[lo])
}

/// Interquartile range of the slice: the third quartile minus the first
/// quartile, `Q3 - Q1`, using linear interpolation. A robust measure of spread
/// that ignores the extreme tails. Panics on empty input.
pub fn iqr(xs: &[f64]) -> f64 {
    assert!(!xs.is_empty(), "iqr of empty slice");
    let mut v = xs.to_vec();
    v.sort_by(f64::total_cmp);
    quantile_sorted(&v, 0.75) - quantile_sorted(&v, 0.25)
}

/// Trimmed mean of the slice: sort the values, drop `floor(frac * n)` elements
/// from each end, and average what remains. `frac` is in `[0.0, 0.5)`. With
/// `frac == 0.0` this is the ordinary arithmetic mean. Panics on empty input
/// or when `frac` is outside `[0.0, 0.5)`.
pub fn trimmed_mean(xs: &[f64], frac: f64) -> f64 {
    assert!(!xs.is_empty(), "trimmed_mean of empty slice");
    assert!((0.0..0.5).contains(&frac), "trimmed_mean frac out of range");
    let mut v = xs.to_vec();
    v.sort_by(f64::total_cmp);
    let n = v.len();
    let k = (frac * n as f64).floor() as usize;
    let core = &v[k..n - k];
    let sum: f64 = core.iter().sum();
    sum / (n - k) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iqr_symmetric() {
        // For 1..=5 the quartiles are Q1 = 2, Q3 = 4, so the IQR is 2.
        assert!((iqr(&[1.0, 2.0, 3.0, 4.0, 5.0]) - 2.0).abs() < 1e-9);
        assert!((iqr(&[5.0, 1.0, 3.0, 2.0, 4.0]) - 2.0).abs() < 1e-9);
    }

    #[test]
    fn iqr_constant_is_zero() {
        assert!(iqr(&[7.0, 7.0, 7.0, 7.0]).abs() < 1e-9);
    }

    #[test]
    fn trimmed_mean_no_trim_is_mean() {
        // frac = 0 trims nothing, so the trimmed mean is the plain mean.
        assert!((trimmed_mean(&[1.0, 2.0, 3.0, 4.0], 0.0) - 2.5).abs() < 1e-9);
        assert!((trimmed_mean(&[10.0, 20.0, 30.0], 0.0) - 20.0).abs() < 1e-9);
    }

    #[test]
    fn trimmed_mean_small_sample_no_trim() {
        // With n = 3 and frac = 0.2 the trim count floors to 0, so again this
        // reduces to the arithmetic mean.
        assert!((trimmed_mean(&[1.0, 2.0, 9.0], 0.2) - 4.0).abs() < 1e-9);
    }

    #[test]
    #[should_panic]
    fn trimmed_mean_empty_panics() {
        trimmed_mean(&[], 0.1);
    }
}
