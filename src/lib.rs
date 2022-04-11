mod digits;
pub use digits::Digits;
use num::{traits::real::Real, FromPrimitive, One};

pub mod fraction;

pub mod series;
pub mod stats;

pub struct Limit {
    iterations: usize,
}

impl Default for Limit {
    fn default() -> Self {
        Self::new(1_000_000)
    }
}

impl Limit {
    pub const MAX: Self = Self::new(usize::MAX);

    pub const fn new(iterations: usize) -> Self {
        Self { iterations }
    }
}

impl From<Limit> for usize {
    fn from(limit: Limit) -> Self {
        limit.iterations
    }
}

pub fn ldexp(x: u32, exp: u32) -> u32 {
    x * 2u32.pow(exp)
}

pub fn epsilon<T: One + FromPrimitive + Real>() -> T {
    T::one() * T::from_u8(2).unwrap().powi(1 - (0f64.digits() as i32))
}

#[cfg(test)]
mod tests {
    use crate::stats::{ Distribution, NormalDistribution, ErrorFunction};

    #[test]
    fn it_works() {
        dbg!(ErrorFunction::default().error(5.));

        dbg!(NormalDistribution::standard().cdf(&0.2, ErrorFunction::default()));
    }
}
