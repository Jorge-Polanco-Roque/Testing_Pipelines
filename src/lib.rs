//! Small statistics helpers.

pub mod bounds;
pub mod measures;
pub mod normalize;
pub mod range;
pub mod summary;

pub use bounds::Quartiles;
pub use measures::{mean, median, percentile, stddev, variance};
pub use normalize::{min_max, z_score};
pub use range::{max, midrange, min, peak_to_peak};
pub use summary::Summary;
