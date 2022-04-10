use num::{traits::real::Real, FromPrimitive, Integer, One, Zero, ToPrimitive};
use std::ops::{Add, AddAssign, Mul, Sub};

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

impl<T> From<Ratio<T>> for f64 where T: ToPrimitive{
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
    T: FromPrimitive + AddAssign + Clone,
{
    type Value = T;

    fn next(&mut self) -> Ratio<Self::Value> {
        self.k += 1;
        self.z += T::from_u8(2).unwrap();

        let k = T::from_i32(self.k).unwrap();
        Ratio::new(&k * &(&self.a - &k), self.z.clone())
    }
}

pub fn continued_fraction_a<F>(mut fraction: F, factor: F::Value, max_terms: usize) -> F::Value
where
    F: Fraction,
    F::Value: One + Zero + Real + Tiny + Clone,
{
    let terminator = factor.abs();
    let mut v = fraction.next();

    let mut f = v.numer().clone();
    let a0 = v.denom().clone();

    if f.is_zero() {
        f = F::Value::tiny();
    }

    let mut c = f;
    let mut d = F::Value::zero();

    let mut counter = max_terms;

    let mut delta;
    loop {
        v = fraction.next();
        d = v.numer().clone() + v.denom().clone() * d;
        if d.is_zero() {
            d = F::Value::tiny();
        }

        c = v.numer().clone() + v.denom().clone() / c;
        if c.is_zero() {
            c = F::Value::tiny();
        }

        delta = c * d;
        f = f * delta;

        if !((delta - F::Value::one()).abs() > terminator && counter > 1) {
            break;
        }

        counter -= 1;
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

pub fn upper_gamma_fraction<T>(a: T, z: T, eps: T) -> T
where
    for<'t> &'t T: Sub<&'t T, Output = T> + Mul<&'t T, Output = T>,
    T: Real + Tiny + FromPrimitive + One + AddAssign + Add<Output = T>,
{
    // Multiply result by z^a * e^-z to get the full
    // upper incomplete integral.  Divide by tgamma(z)
    // to normalise.
    let f = UpperIncompleteGammaFraction::new(a, z);

    T::one() / (z - a + T::one() + continued_fraction_a(f, eps, 10000000))
}
