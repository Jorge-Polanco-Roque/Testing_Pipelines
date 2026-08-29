//! Bivariate statistics over two paired slices.

/// Population covariance of the paired slices `xs` and `ys`:
///
/// ```text
/// cov = ( sum((x - mean_x) * (y - mean_y)) ) / n
/// ```
///
/// This is the population form (divisor `n`, matching the population variance
/// used elsewhere in the crate), so `covariance(xs, xs)` equals the population
/// variance of `xs`. It is zero for uncorrelated data, positive when the two
/// series move together and negative when they move oppositely. Panics on empty
/// input or when the lengths differ.
pub fn covariance(xs: &[f64], ys: &[f64]) -> f64 {
    assert!(!xs.is_empty(), "covariance of empty slice");
    assert_eq!(xs.len(), ys.len(), "covariance length mismatch");
    let n = xs.len() as f64;
    let mx: f64 = xs.iter().sum::<f64>() / n;
    let my: f64 = ys.iter().sum::<f64>() / n;
    let acc: f64 = xs.iter().zip(ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    // population divisor n, matches crate-wide population convention.
    acc / n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn covariance_uncorrelated_is_zero() {
        // Symmetric pairing: the cross-deviation sum is exactly 0, so any positive
        // divisor gives 0. Divisor choice is not exercised here.
        let xs = [1.0, 2.0, 3.0, 4.0, 5.0];
        let ys = [2.0, 2.0, 2.0, 2.0, 2.0];
        assert!(covariance(&xs, &ys).abs() < 1e-9);
    }

    #[test]
    fn covariance_constant_series_is_zero() {
        // A constant y series has zero deviations, so covariance is 0 regardless
        // of the divisor.
        assert!(covariance(&[3.0, 1.0, 4.0, 1.0], &[7.0, 7.0, 7.0, 7.0]).abs() < 1e-9);
    }

    #[test]
    fn covariance_sign_positive() {
        // Perfectly increasing together: covariance must be strictly positive.
        // We only assert the sign, not the magnitude.
        let xs = [1.0, 2.0, 3.0];
        let ys = [10.0, 20.0, 30.0];
        assert!(covariance(&xs, &ys) > 0.0);
    }

    #[test]
    fn covariance_sign_negative() {
        // Moving oppositely: covariance must be strictly negative (sign only).
        let xs = [1.0, 2.0, 3.0];
        let ys = [30.0, 20.0, 10.0];
        assert!(covariance(&xs, &ys) < 0.0);
    }

    #[test]
    fn covariance_symmetric_in_args() {
        // cov(x, y) == cov(y, x) whatever the divisor.
        let xs = [1.0, 3.0, 2.0, 8.0];
        let ys = [2.0, 1.0, 5.0, 4.0];
        assert!((covariance(&xs, &ys) - covariance(&ys, &xs)).abs() < 1e-9);
    }

    #[test]
    fn covariance_population_magnitude() {
        // Population divisor n: cov(x, x) equals population variance of x.
        let x = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert!((covariance(&x, &x) - 2.0).abs() < 1e-9);
        assert!((covariance(&[1.0, 2.0, 3.0], &[2.0, 4.0, 6.0]) - 4.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    #[should_panic]
    fn covariance_empty_panics() {
        covariance(&[], &[]);
    }

    #[test]
    #[should_panic]
    fn covariance_length_mismatch_panics() {
        covariance(&[1.0, 2.0], &[1.0]);
    }
}
