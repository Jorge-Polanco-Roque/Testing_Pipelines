//! Cumulative sequences: prefix sums and running maxima.

/// Cumulative (prefix) sum of the slice. Element `i` of the result is the sum
/// of `xs[0..=i]`, so the last element equals the total sum. Returns an empty
/// vector for empty input.
pub fn cumsum(xs: &[f64]) -> Vec<f64> {
    let mut acc = 0.0;
    xs.iter()
        .map(|x| {
            acc += x;
            acc
        })
        .collect()
}

/// Running (prefix) maximum of the slice. Element `i` of the result is the
/// largest value seen in `xs[0..=i]`, so the sequence is non-decreasing and its
/// last element equals the overall maximum. Returns an empty vector for empty
/// input.
pub fn running_max(xs: &[f64]) -> Vec<f64> {
    if xs.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(xs.len());
    out.push(xs[0]);
    for i in 1..xs.len() {
        // Carry the running maximum forward, comparing against the prefix max so far.
        out.push(xs[i].max(out[i - 1]));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cumsum_basic() {
        assert_eq!(cumsum(&[1.0, 2.0, 3.0, 4.0]), vec![1.0, 3.0, 6.0, 10.0]);
    }

    #[test]
    fn cumsum_empty() {
        assert_eq!(cumsum(&[]), Vec::<f64>::new());
    }

    #[test]
    fn running_max_monotonic() {
        // Non-decreasing input: each element is its own prefix maximum, so a
        // one-step look-back and a true running maximum agree everywhere.
        assert_eq!(running_max(&[1.0, 2.0, 3.0, 4.0]), vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(running_max(&[0.0, 0.0, 5.0, 5.0]), vec![0.0, 0.0, 5.0, 5.0]);
    }

    #[test]
    fn running_max_single_peak() {
        // A rise immediately followed by a single value: the peak still holds
        // on the next step under either interpretation.
        assert_eq!(running_max(&[1.0, 4.0, 2.0]), vec![1.0, 4.0, 4.0]);
    }

    #[test]
    fn running_max_dip_after_peak() {
        // A dip after the peak must not lower the running maximum; the prefix
        // max is carried forward and the last element equals the global max.
        assert_eq!(running_max(&[1.0, 5.0, 2.0, 3.0]), vec![1.0, 5.0, 5.0, 5.0]);
    }

    #[test]
    fn running_max_empty() {
        assert_eq!(running_max(&[]), Vec::<f64>::new());
    }
}
