//! Continued fractions

use num::{traits::real::Real, One, ToPrimitive, Zero};

mod gamma;
pub use gamma::{upper_gamma_fraction, UpperIncompleteGammaFraction};

pub trait Fraction: IntoIterator<Item = Ratio<Self::Value>> + Sized {
    type Value: One + Zero + Real + Tiny + Clone;

    fn continued_fraction_a(self, factor: Self::Value) -> Self::Value {
        let mut fraction = self.into_iter();
        let terminator = factor.abs();
        let mut v = fraction.next().unwrap();

        let mut f = v.numer().clone();
        let a0 = v.denom().clone();

        if f.is_zero() {
            f = Self::Value::tiny();
        }

        let mut c = f;
        let mut d = Self::Value::zero();

        let mut delta;

        while let Some(next) = fraction.next() {
            v = next;

            d = v.numer().clone() + v.denom().clone() * d;
            if d.is_zero() {
                d = Self::Value::tiny();
            }

            c = v.numer().clone() + v.denom().clone() / c;
            if c.is_zero() {
                c = Self::Value::tiny();
            }

            delta = c * d;
            f = f * delta;

            if (delta - Self::Value::one()).abs() <= terminator {
                break;
            }
        }

        a0 / f
    }
}

impl<I, T> Fraction for I
where
    I: IntoIterator<Item = Ratio<T>>,
    T: One + Zero + Real + Tiny + Clone,
{
    type Value = T;
}

pub trait Tiny {
    fn tiny() -> Self;
}

impl Tiny for f64 {
    fn tiny() -> Self {
        16. * Self::MIN
    }
}

pub struct Ratio<T> {
    n: T,
    d: T,
}

impl<T> Ratio<T> {
    pub fn new(n: T, d: T) -> Self {
        Self { n, d }
    }

    pub fn numer(&self) -> &T {
        &self.n
    }

    pub fn denom(&self) -> &T {
        &self.d
    }
}

impl<T> From<Ratio<T>> for f64
where
    T: ToPrimitive,
{
    fn from(ratio: Ratio<T>) -> Self {
        ratio.numer().to_f64().unwrap() / ratio.denom().to_f64().unwrap()
    }
}
