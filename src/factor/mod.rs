mod prime_factors;
pub use prime_factors::PrimeFactors;

use num::{Num, One};

/// ```
/// use quickmaths::factor::Factors;
///
/// assert!(Factors::new(1).eq([1]));
/// assert!(Factors::new(5).eq([1, 5]));
/// assert!(Factors::new(24).eq([1, 2, 3, 4, 6, 8, 12, 24]));
/// assert!(Factors::new(-24).eq([]));
/// ```
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
