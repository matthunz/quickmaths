use num::{FromPrimitive, One};
use std::ops::{Add, AddAssign, Mul, Sub};

pub trait Fraction {
    type Value;

    fn next(&mut self) -> (Self::Value, &Self::Value);
}

pub struct UpperIncompleteGammaFraction<T> {
    z: T,
    a: T,
    k: i32,
}

impl<T> UpperIncompleteGammaFraction<T>
where
    for<'t> &'t T: Sub<&'t T, Output = T> + Mul<&'t T, Output = T>,
    T: FromPrimitive + One + AddAssign + Add<Output = T>,
{
    pub fn new(a1: T, z1: T) -> Self {
        Self {
            z: &z1 - &a1 + T::one(),
            a: a1,
            k: 0,
        }
    }
}

impl<T> Fraction for UpperIncompleteGammaFraction<T>
where
    for<'t> &'t T: Sub<&'t T, Output = T> + Mul<&'t T, Output = T>,
    T: FromPrimitive + One + AddAssign + Add<Output = T>,
{
    type Value = T;

    fn next(&mut self) -> (Self::Value, &Self::Value) {
        self.k += 1;
        self.z += T::from_u8(2).unwrap();

        let k = T::from_i32(self.k).unwrap();
        (&k * &(&self.a - &k), &self.z)
    }
}
