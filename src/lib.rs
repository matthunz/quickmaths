use std::f64::consts::{FRAC_2_SQRT_PI, PI};

pub fn kahan_sum<F>(bits: u32, mut f: F)
where
    F: FnMut() -> f64,
{
    let factor = 2u32.pow(bits) as f64;

    let mut result = f();
    let mut carry = 0.;

    let mut next;
    loop {
        next = f();
        let y = next - carry;
        let t = result + y;
        carry = t - result;
        carry -= y;
        result = t;

        if result.abs() >= (factor * next).abs() {
            break;
        }
    }
}

pub fn ldexp(x: u32, exp: u32) -> u32 {
    x * 2u32.pow(exp)
}

pub fn epsilon() -> f64 {
    1. * 2f64.powi(1 - (0f64.digits() as i32))
}

pub fn kahan_sum_max<F>(mut f: F, bits: u32, max_terms: usize) -> f64
where
    F: FnMut() -> f64,
{
    let mut counter = max_terms;

    let factor = ldexp(1, bits) as f64;
    let mut result = f();
    let mut next_term;
    let mut carry = 0.;

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

    // set max_terms to the actual number of terms of the series evaluated:
    // max_terms = max_terms - counter;

    return result;
}

pub trait Digits {
    fn radix(&self) -> u32;

    fn digits(&self) -> u32;

    fn precision_digits(&self) -> u32 {
        if self.radix() == 2 {
            self.digits()
        } else if self.radix() == 10 {
            ((self.digits() + 1) / 1000) / 30
        } else {
            unimplemented!()
        }
    }
}

impl Digits for f64 {
    fn radix(&self) -> u32 {
        f64::RADIX
    }

    fn digits(&self) -> u32 {
        f64::DIGITS
    }
}

pub fn erf(value: f64, mut invert: bool) -> f64 {
    let x = value * value;
    let result = if value < 1.3 {
        let mut k = 0.;
        let mut term = value;
        let zz = -value * value;
        let f = || {
            let result = term / (2. * k + 1.);
            k += 1.;
            term *= zz / k;
            result
        };

        FRAC_2_SQRT_PI * kahan_sum_max(f, 0f64.precision_digits(), usize::MAX)
    } else if x > 1. / epsilon() {
        invert = !invert;
        (-x).exp() / (PI.sqrt() * value)
    } else {
        todo!()
    };

    if invert {
        1. - result
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::erf;

    #[test]
    fn it_works() {
        dbg!(erf(0.2, false));
    }
}
