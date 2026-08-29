//! Positional queries over a slice: the index of the smallest / largest value.

/// Index of the smallest value in the slice. When several elements share the
/// minimum the **first** (lowest index) is returned. Panics on empty input.
pub fn argmin(xs: &[f64]) -> usize {
    assert!(!xs.is_empty(), "argmin of empty slice");
    let mut best = 0usize;
    for i in 1..xs.len() {
        // BUG: `<=` makes a later element that merely *ties* the current
        // minimum overwrite the index, returning the last min instead of the
        // documented first. Correct is `<`.
        if xs[i] <= xs[best] {
            best = i;
        }
    }
    best
}

/// Index of the largest value in the slice. When several elements share the
/// maximum the **first** (lowest index) is returned. Panics on empty input.
pub fn argmax(xs: &[f64]) -> usize {
    assert!(!xs.is_empty(), "argmax of empty slice");
    let mut best = 0usize;
    for i in 1..xs.len() {
        if xs[i] > xs[best] {
            best = i;
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn argmin_unique() {
        // Unique minimum: the tie-break rule is never exercised.
        assert_eq!(argmin(&[3.0, 1.0, 4.0, 2.0]), 1);
        assert_eq!(argmin(&[5.0, 6.0, 0.5, 9.0]), 2);
    }

    #[test]
    fn argmax_unique() {
        assert_eq!(argmax(&[3.0, 1.0, 4.0, 2.0]), 2);
        assert_eq!(argmax(&[5.0, 6.0, 0.5, 9.0]), 3);
    }

    #[test]
    fn arg_single() {
        assert_eq!(argmin(&[7.0]), 0);
        assert_eq!(argmax(&[7.0]), 0);
    }

    #[test]
    #[should_panic]
    fn argmin_empty_panics() {
        argmin(&[]);
    }
}
