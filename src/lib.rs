mod digits;

pub use digits::Digits;
use num::{traits::real::Real, FromPrimitive, Integer, One, Zero};

pub mod factor;

pub mod fraction;

pub mod integral;

pub mod series;
pub mod stats;

pub mod poly;

pub fn ldexp(x: u32, exp: u32) -> u32 {
    x * 2u32.pow(exp)
}

pub fn epsilon<T: One + FromPrimitive + Real>() -> T {
    T::one() * T::from_u8(2).unwrap().powi(1 - (0f64.digits() as i32))
}

pub fn gcd<I>(integers: I) -> I::Item
where
    I: IntoIterator,
    I::Item: Integer,
{
    let mut iter = integers.into_iter();
    iter.next()
        .map(|i| iter.fold(i, |acc, i| acc.gcd(&i)))
        .unwrap_or_else(|| I::Item::zero())
}

#[cfg(test)]
mod tests {
    use crate::{
        poly::Polynomial,
        stats::{Distribution, ErrorFunction, NormalDistribution},
    };

    #[test]
    fn it_works() {
        dbg!(ErrorFunction::default().error(5.));

        dbg!(NormalDistribution::standard().cdf(&0.2, ErrorFunction::default()));

        dbg!([0., 4.].integral().collect::<Vec<_>>());
    }
}
