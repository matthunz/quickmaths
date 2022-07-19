use super::{Fraction, Ratio, Tiny};
use num::{traits::real::Real, FromPrimitive, One};
use std::ops::{Add, Mul, Sub};

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

pub fn upper_gamma_fraction<T>(a: T, z: T, eps: T, max_iters: usize) -> T
where
    for<'t> &'t T: Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    T: Real + Tiny + FromPrimitive + One,
{
    // Multiply result by z^a * e^-z to get the full
    // upper incomplete integral.  Divide by tgamma(z)
    // to normalise.
    let frac = UpperIncompleteGammaFraction::new(a, z).take(max_iters);

    T::one() / (z - a + T::one() + frac.continued_fraction_a(eps))
}
