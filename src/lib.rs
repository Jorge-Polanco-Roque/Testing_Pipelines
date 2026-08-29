//! Small statistics helpers.

pub mod bivariate;
pub mod bounds;
pub mod measures;
pub mod moments;
pub mod normalize;
pub mod range;
pub mod rank;
pub mod summary;
pub mod weighted;

pub use bivariate::covariance;
pub use bounds::Quartiles;
pub use measures::{mean, median, mode, percentile, stddev, variance};
pub use moments::skewness;
pub use normalize::{min_max, z_score};
pub use range::{max, midrange, min, peak_to_peak};
pub use rank::{argmax, argmin};
pub use summary::Summary;
pub use weighted::weighted_mean;
