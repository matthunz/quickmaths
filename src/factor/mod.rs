//! Factorization and primes

mod prime_factors;
pub use prime_factors::PrimeFactors;

use num::{integer::Roots, range, FromPrimitive, Integer, Num, One, ToPrimitive, Unsigned};

/// ```
/// use quickmaths::factor::gcd;
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

    /// ```
    /// use quickmaths::Factor;
    ///
    /// assert!(0u32.prime_factors().eq([]));
    /// assert!(11u32.prime_factors().eq([11]));
    /// assert!(25u32.prime_factors().eq([5, 5]));
    /// assert!(33u32.prime_factors().eq([3, 11]));
    /// assert!(2560u32.prime_factors().eq([2, 2, 2, 2, 2, 2, 2, 2, 2, 5]));
    /// ```
    fn prime_factors(self) -> PrimeFactors<Self>
    where
        Self: Unsigned + FromPrimitive + PartialOrd,
    {
        PrimeFactors::new(self)
    }

    /// ```
    /// use quickmaths::Factor;
    ///
    /// assert!(3.is_prime());
    /// assert!(!4.is_prime());
    /// ```
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
