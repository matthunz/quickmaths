//! Series

use crate::{fraction::Ratio, ldexp, Digits};
use num::{traits::real::Real, FromPrimitive, Unsigned, Zero};

/// ```
/// use quickmaths::series::kahan_sum;
///
/// assert_eq!(kahan_sum([0.1; 10]), 1.);
/// ```
pub fn kahan_sum<T>(iter: T) -> T::Item
where
    T: IntoIterator,
    T::Item: Digits + Zero + FromPrimitive + Real,
{
    let mut iter = iter.into_iter();
    let mut result = if let Some(term) = iter.next() {
        term
    } else {
        return T::Item::zero();
    };

    let mut carry = T::Item::zero();
    let factor = T::Item::from_u32(ldexp(1, carry.precision_digits())).unwrap();

    while let Some(term) = iter.next() {
        let y = term - carry;
        let t = result + y;
        carry = t - result;
        carry = carry - y;
        result = t;

        if result.abs() >= (factor * term).abs() {
            break;
        }
    }

    result
}

/// Iterator for the [harmonic series](https://en.wikipedia.org/wiki/Harmonic_series_(mathematics)).
/// `Iterator::nth` skips directly to the `n`th term.
pub struct HarmonicSeries<T> {
    pub i: T,
}

impl<T> HarmonicSeries<T>
where
    T: Unsigned + FromPrimitive + Clone,
{
    // Create a new harmonic series starting at `i`.
    pub fn new(i: T) -> Self {
        Self { i }
    }
}

impl<T> Default for HarmonicSeries<T>
where
    T: Unsigned + FromPrimitive + Clone,
{
    fn default() -> Self {
        Self::new(T::one())
    }
}

impl<T> Iterator for HarmonicSeries<T>
where
    T: Unsigned + FromPrimitive + Clone,
{
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let frac = Ratio::new(T::one(), self.i.clone());
        self.i = self.i.clone() + T::one();
        Some(frac)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.i = T::from_usize(n - 1).unwrap();
        self.next()
    }
}
