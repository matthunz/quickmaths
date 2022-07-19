#![cfg_attr(not(feature = "std"), no_std)]

mod digits;
pub use digits::Digits;

pub mod factor;

pub mod fraction;

pub mod integral;

pub mod series;
pub mod stats;

pub mod poly;

use num::{traits::real::Real, FromPrimitive, Integer, One};

pub fn ldexp(x: u32, exp: u32) -> u32 {
    x * 2u32.pow(exp)
}

pub fn epsilon<T: One + FromPrimitive + Real>() -> T {
    T::one() * T::from_u8(2).unwrap().powi(1 - (0f64.digits() as i32))
}

/// ```
/// use quickmaths::gcd;
/// 
/// assert_eq!(gcd([1, 2]), 1);
/// assert_eq!(gcd([3, 6]), 3);
/// ```
pub fn gcd<I>(integers: I) -> I::Item
where
    I: IntoIterator,
    I::Item: Integer,
{
    let mut iter = integers.into_iter();
    iter.next()
        .map(|i| iter.fold(i, |acc, i| acc.gcd(&i)))
        .unwrap_or_else(|| I::Item::one())
}
