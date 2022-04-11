use num::{FromPrimitive, Integer, Zero};
use std::ops::Div;

pub trait Polynomial: IntoIterator + Sized {
    /// ```
    /// use quickmaths::poly::Polynomial;
    ///
    /// let poly = [0., 4.];
    /// assert!(poly.integral().eq([0., 0., 2.]));
    /// ```
    fn integral(self) -> Integral<Self::IntoIter> {
        Integral {
            iter: self.into_iter(),
            pos: 0,
        }
    }

    fn content(self) -> Self::Item
    where
        Self::Item: Integer,
    {
        crate::gcd(self)
    }
}

impl<T> Polynomial for T where T: IntoIterator {}

pub struct Integral<T: Iterator> {
    iter: T,
    pos: usize,
}

impl<T> Iterator for Integral<T>
where
    T: Iterator,
    T::Item: Zero + FromPrimitive + Div<Output = T::Item>,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let coeff = if self.pos == 0 {
            Some(T::Item::zero())
        } else {
            self.iter
                .next()
                .map(|c| c / T::Item::from_usize(self.pos).unwrap())
        };
        self.pos += 1;

        coeff
    }
}
