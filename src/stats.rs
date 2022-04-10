use crate::{epsilon, series::kahan_sum, Digits};
use std::f64::consts::{FRAC_2_SQRT_PI, PI};

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

        FRAC_2_SQRT_PI * kahan_sum(f, 0f64.precision_digits(), Some(usize::MAX))
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
