use super::{Distribution, ErrorFunction};
use crate::{
    fraction::{Ratio, Tiny},
    Digits,
};
use num::{
    traits::{real::Real, FloatConst},
    FromPrimitive, One, Zero,
};
use std::ops::{Add, Mul, Sub};

pub struct NormalDistribution<T> {
    mean: T,
    std_deviation: T,
}

impl<T> NormalDistribution<T> {
    pub fn new(mean: T, std_deviation: T) -> Self {
        Self {
            mean,
            std_deviation,
        }
    }
}

impl<T> NormalDistribution<T>
where
    T: One + Zero,
{
    pub fn standard() -> Self {
        Self::new(T::zero(), T::one())
    }
}

impl<T> Distribution for NormalDistribution<T>
where
    T: Tiny + Digits + FromPrimitive + FloatConst + Real + From<Ratio<i32>> + PartialOrd,
    for<'t> &'t T: Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    type Value = T;

    fn cdf(&self, x: &Self::Value, error: ErrorFunction) -> Self::Value {
        let one_half: T = Ratio::new(1, 2).into();
        one_half * error.complementary_error((&self.mean - x) / (self.std_deviation * T::SQRT_2()))
    }
}
