use crate::{
    epsilon,
    fraction::{upper_gamma_fraction, Ratio, Tiny},
    series::kahan_sum,
    Digits,
};
use num::{
    traits::{real::Real, FloatConst},
    FromPrimitive,
};
use std::{
    iter,
    ops::{Add, Mul, Sub},
};

mod normal;
pub use normal::NormalDistribution;

pub trait Distribution {
    type Value;

    fn cdf(&self, x: &Self::Value, error: ErrorFunction) -> Self::Value;
}

pub struct ErrorFunction {
    pub sum_max_iters: usize,
    pub fraction_max_iters: usize,
}

impl Default for ErrorFunction {
    fn default() -> Self {
        Self {
            sum_max_iters: 1_000_000,
            fraction_max_iters: 1_000_000,
        }
    }
}

impl ErrorFunction {
    /// Calculates the error function at `x`.
    pub fn error<T>(self, x: T) -> T
    where
        T: Tiny + Digits + FromPrimitive + FloatConst + Real + From<Ratio<i32>> + PartialOrd,
        for<'t> &'t T: Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        self.error_inner(x, false)
    }

    /// Calculates the complementary error function at `x`.
    pub fn complementary_error<T>(self, x: T) -> T
    where
        T: Tiny + Digits + FromPrimitive + FloatConst + Real + From<Ratio<i32>> + PartialOrd,
        for<'t> &'t T: Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        self.error_inner(x, true)
    }

    fn error_inner<T>(self, value: T, mut invert: bool) -> T
    where
        T: Tiny + Digits + FromPrimitive + FloatConst + Real + From<Ratio<i32>> + PartialOrd,
        for<'t> &'t T: Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        let x = value * value;
        let result = if value < Ratio::new(13, 10).into() {
            let mut k = T::zero();
            let mut term = value;
            let zz = -value * value;
            let f = iter::from_fn(|| {
                let result = term / (T::from_u8(2).unwrap() * k + T::one());
                k = k + T::one();
                term = term * (zz / k);
                Some(result)
            });

            T::FRAC_2_SQRT_PI() * kahan_sum(f.take(self.sum_max_iters.into()))
        } else if x > T::one() / epsilon() {
            invert = !invert;
            (-x).exp() / (T::PI().sqrt() * value)
        } else {
            invert = !invert;

            ((value * (-x).exp()) / T::PI().sqrt())
                * upper_gamma_fraction(
                    Ratio::new(1, 2).into(),
                    x,
                    epsilon(),
                    self.fraction_max_iters,
                )
        };

        if invert {
            T::one() - result
        } else {
            result
        }
    }
}
