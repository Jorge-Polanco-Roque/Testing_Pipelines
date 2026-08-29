//! Small statistics helpers.

pub mod bivariate;
pub mod bounds;
pub mod cumulative;
pub mod means;
pub mod measures;
pub mod moments;
pub mod normalize;
pub mod range;
pub mod rank;
pub mod spread;
pub mod summary;
pub mod weighted;

pub use bivariate::covariance;
pub use bounds::Quartiles;
pub use cumulative::{cumsum, running_max};
pub use means::{geometric_mean, harmonic_mean};
pub use measures::{mean, median, mode, percentile, stddev, variance};
pub use moments::skewness;
pub use normalize::{min_max, z_score};
pub use range::{max, midrange, min, peak_to_peak};
pub use rank::{argmax, argmin};
pub use spread::{iqr, trimmed_mean};
pub use summary::Summary;
pub use weighted::weighted_mean;
