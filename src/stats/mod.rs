//! Statistics

mod error;
pub use error::ErrorFunction;

mod normal;
pub use normal::NormalDistribution;

pub trait Distribution {
    type Value;

    fn cdf(&self, x: &Self::Value, error: ErrorFunction) -> Self::Value;
}
