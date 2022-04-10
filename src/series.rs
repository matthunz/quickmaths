use crate::ldexp;
use num::{traits::real::Real, FromPrimitive};
use std::ops::SubAssign;

pub fn kahan_sum<T, F>(mut f: F, bits: u32, max_terms: Option<usize>) -> T
where
    F: FnMut() -> T,
    T: FromPrimitive + Real + SubAssign,
{
    if let Some(max_terms) = max_terms {
        let mut counter = max_terms;

        let factor = T::from_u32(ldexp(1, bits)).unwrap();
        let mut result = f();
        let mut next_term;
        let mut carry = T::zero();

        loop {
            next_term = f();
            let y = next_term - carry;
            let t = result + y;
            carry = t - result;
            carry -= y;
            result = t;

            if !((result.abs() < (factor * next_term).abs()) && counter > 1) {
                counter -= 1;
                break;
            }
        }
        result
    } else {
        let factor = T::from_u32(2u32.pow(bits)).unwrap();

        let mut result = f();
        let mut carry = T::zero();

        let mut next;
        loop {
            next = f();
            let y = next - carry;
            let t = result + y;
            carry = t - result;
            carry -= y;
            result = t;

            if result.abs() >= (factor * next).abs() {
                break result;
            }
        }
    }
}
