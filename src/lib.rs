//! A collection of algorithms for generic mathematics in Rust.

#![cfg_attr(not(feature = "std"), no_std)]

mod digits;
pub use digits::Digits;

pub mod factor;
pub use factor::Factor;

pub mod fraction;

pub mod integral;

pub mod series;
pub mod stats;

pub mod poly;

use num::{traits::real::Real, FromPrimitive, One};

pub fn ldexp(x: u32, exp: u32) -> u32 {
    x * 2u32.pow(exp)
}

pub fn epsilon<T: One + FromPrimitive + Real>() -> T {
    T::one() * T::from_u8(2).unwrap().powi(1 - (0f64.digits() as i32))
}
