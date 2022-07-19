//! Factorization and primes

mod prime_factors;
pub use prime_factors::PrimeFactors;

use num::{integer::Roots, range, FromPrimitive, Integer, Num, One, ToPrimitive, Unsigned};

pub trait Factor: Clone {
    /// ```
    /// use quickmaths::Factor;
    ///
    /// assert!(1.factors().eq([1]));
    /// assert!(5.factors().eq([1, 5]));
    /// assert!(24.factors().eq([1, 2, 3, 4, 6, 8, 12, 24]));
    /// assert!((-24).factors().eq([]));
    /// ```
    fn factors(self) -> Factors<Self>
    where
        Self: Integer + Copy,
    {
        Factors::new(self)
    }

    fn prime_factors(self) -> PrimeFactors<Self>
    where
        Self: Unsigned + FromPrimitive + PartialOrd,
    {
        PrimeFactors::new(self)
    }

    fn is_prime(self) -> bool
    where
        Self: Integer + Roots + FromPrimitive + ToPrimitive,
    {
        if (self > Self::one()) & (self < Self::from_u8(4).unwrap()) {
            return true;
        } else if (self < Self::from_u8(2).unwrap())
            || (self.clone() % Self::from_u8(2).unwrap()).is_zero()
        {
            return false;
        }

        let end = self.sqrt() + Self::one();
        for i in range(Self::from_u8(3).unwrap(), end).step_by(2) {
            if (self.clone() % i).is_zero() {
                return false;
            }
        }
        true
    }
}

impl<T: Clone> Factor for T {}

pub struct Factors<T> {
    n: T,
    i: T,
}

impl<T: One> Factors<T> {
    pub fn new(n: T) -> Self {
        Self { n, i: T::one() }
    }
}

impl<T> Iterator for Factors<T>
where
    T: Num + PartialOrd + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while self.i <= self.n {
            let i = self.i.clone();
            self.i = self.i.clone() + T::one();
            if (self.n.clone() % i.clone()).is_zero() {
                return Some(i);
            }
        }
        None
    }
}
