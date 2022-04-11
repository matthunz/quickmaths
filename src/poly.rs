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

    /// ```
    /// use quickmaths::poly::Polynomial;
    ///
    /// let poly = [-20, 30, 0, -12];
    /// assert_eq!(poly.content(), 2);
    /// ```
    fn content(self) -> Self::Item
    where
        Self::Item: Integer,
    {
        crate::gcd(self)
    }

    /// ```
    /// use quickmaths::poly::Polynomial;
    ///
    /// let poly = [-20, 30, 0, -12];
    /// assert!(poly.primitive_part().eq([-10, 15, 0, -6]));
    /// ```
    fn primitive_part(self) -> Divide<Self::IntoIter, Self::Item>
    where
        Self::IntoIter: Clone,
        Self::Item: Integer,
    {
        let iter = self.into_iter();
        iter.clone().div(iter.content())
    }

    fn div<U>(self, divisor: U) -> Divide<Self::IntoIter, U> {
        Divide {
            iter: self.into_iter(),
            divisor,
        }
    }
}

impl<T> Polynomial for T where T: IntoIterator {}

pub struct Divide<T, U> {
    iter: T,
    divisor: U,
}

impl<T, U> Iterator for Divide<T, U>
where
    T: Iterator,
    T::Item: for<'u> Div<&'u U, Output = T::Item>,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|lhs| lhs / &self.divisor)
    }
}

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
