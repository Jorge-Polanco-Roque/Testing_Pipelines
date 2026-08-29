//! Power-average helpers: the root mean square (quadratic mean).

/// Root mean square (quadratic mean) of the slice: the square root of the mean
/// of the squares, `sqrt((1/n) * sum(x_i^2))`.
///
/// The RMS is the magnitude-average used for signals and errors; it weights
/// large-magnitude values more heavily than the arithmetic mean and, unlike it,
/// is unaffected by the sign of the inputs. For any data the RMS is at least the
/// absolute value of the arithmetic mean. Panics on empty input.
pub fn root_mean_square(xs: &[f64]) -> f64 {
    assert!(!xs.is_empty(), "root_mean_square of empty slice");
    let mean_sq: f64 = xs.iter().map(|x| x * x).sum::<f64>() / xs.len() as f64;
    mean_sq.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rms_symmetric_about_zero() {
        // Data symmetric about zero: RMS of [-a, a] is a.
        assert!((root_mean_square(&[-3.0, 3.0]) - 3.0).abs() < 1e-9);
        assert!((root_mean_square(&[-2.0, -1.0, 1.0, 2.0]) - (2.5_f64).sqrt()).abs() < 1e-9);
    }

    #[test]
    fn rms_balanced_pairs() {
        // Values that balance around the origin: RMS of [-1, 1] is 1.
        assert!((root_mean_square(&[-1.0, 1.0]) - 1.0).abs() < 1e-9);
        // [-6, -2, 2, 6] -> mean of squares (36+4+4+36)/4 = 20, RMS = sqrt(20).
        assert!((root_mean_square(&[-6.0, -2.0, 2.0, 6.0]) - (20.0_f64).sqrt()).abs() < 1e-9);
    }

    #[test]
    fn rms_nonnegative() {
        // The RMS is a magnitude and can never be negative.
        assert!(root_mean_square(&[-4.0, 4.0, -1.0, 1.0]) >= 0.0);
    }

    #[test]
    fn rms_non_zero_mean() {
        // Regression: RMS must depend only on magnitudes, not on the mean.
        assert!((root_mean_square(&[3.0, 4.0]) - (12.5_f64).sqrt()).abs() < 1e-9);
        // Constant data [c; n] -> RMS is |c|.
        assert!((root_mean_square(&[5.0, 5.0, 5.0]) - 5.0).abs() < 1e-9);
        // Single value -> its magnitude.
        assert!((root_mean_square(&[-7.0]) - 7.0).abs() < 1e-9);
    }

    #[test]
    fn rms_dominates_abs_mean() {
        // Invariant for any data: RMS >= |arithmetic mean|.
        let cases: &[&[f64]] = &[
            &[3.0, 4.0],
            &[5.0, 5.0, 5.0],
            &[-7.0],
            &[1000.0, 1000.1, 999.9], // large shifted mean
            &[-2.0, 8.0, 3.0, -5.0],
        ];
        for xs in cases {
            let rms = root_mean_square(xs);
            let mean = xs.iter().sum::<f64>() / xs.len() as f64;
            assert!(
                rms + 1e-9 >= mean.abs(),
                "RMS {rms} < |mean| {} for {xs:?}",
                mean.abs()
            );
        }
    }

    #[test]
    #[should_panic]
    fn rms_empty_panics() {
        root_mean_square(&[]);
    }
}
