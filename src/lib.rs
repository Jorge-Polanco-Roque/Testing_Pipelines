//! Small statistics helpers.

pub mod bounds;
pub mod measures;
pub mod summary;

pub use bounds::Quartiles;
pub use measures::{mean, median, percentile, variance};
pub use summary::Summary;
