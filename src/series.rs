//! Series

use crate::{ldexp, Digits};
use num::{traits::real::Real, FromPrimitive, Zero};

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
