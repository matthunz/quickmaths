use num::{traits::real::Real, FromPrimitive, One, ToPrimitive, Zero};
use std::ops::{Add, Mul, Sub};

use crate::Limit;

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

pub trait Fraction {
    type Value;

    fn next(&mut self) -> Ratio<Self::Value>;
}

pub struct UpperIncompleteGammaFraction<T> {
    z: T,
    a: T,
    k: i32,
}

impl<T> UpperIncompleteGammaFraction<T>
where
    for<'t> &'t T: Sub<Output = T> + Add<Output = T>,
    T: One,
{
    pub fn new(a1: T, z1: T) -> Self {
        Self {
            z: &(&z1 - &a1) + &T::one(),
            a: a1,
            k: 0,
        }
    }
}

impl<T> Iterator for UpperIncompleteGammaFraction<T>
where
    for<'t> &'t T: Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    T: FromPrimitive + Clone,
{
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.k += 1;
        self.z = &self.z + &T::from_u8(2).unwrap();

        let k = T::from_i32(self.k).unwrap();
        Some(Ratio::new(&k * &(&self.a - &k), self.z.clone()))
    }
}

pub fn continued_fraction_a<T, F>(fraction: F, factor: T) -> T
where
    F: IntoIterator<Item = Ratio<T>>,
    T: One + Zero + Real + Tiny + Clone,
{
    let mut fraction = fraction.into_iter();
    let terminator = factor.abs();
    let mut v = fraction.next().unwrap();

    let mut f = v.numer().clone();
    let a0 = v.denom().clone();

    if f.is_zero() {
        f = T::tiny();
    }

    let mut c = f;
    let mut d = T::zero();

    let mut delta;

    while let Some(next) = fraction.next() {
        v = next;

        d = v.numer().clone() + v.denom().clone() * d;
        if d.is_zero() {
            d = T::tiny();
        }

        c = v.numer().clone() + v.denom().clone() / c;
        if c.is_zero() {
            c = T::tiny();
        }

        delta = c * d;
        f = f * delta;

        if (delta - T::one()).abs() <= terminator {
            break;
        }
    }

    a0 / f
}

pub trait Tiny {
    fn tiny() -> Self;
}

impl Tiny for f64 {
    fn tiny() -> Self {
        16. * Self::MIN
    }
}

pub fn upper_gamma_fraction<T>(a: T, z: T, eps: T, limit: Limit) -> T
where
    for<'t> &'t T: Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    T: Real + Tiny + FromPrimitive + One,
{
    // Multiply result by z^a * e^-z to get the full
    // upper incomplete integral.  Divide by tgamma(z)
    // to normalise.
    let f = UpperIncompleteGammaFraction::new(a, z).take(limit.into());

    T::one() / (z - a + T::one() + continued_fraction_a(f, eps))
}
