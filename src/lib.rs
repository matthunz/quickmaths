mod digits;
pub use digits::Digits;

pub mod fraction;

pub mod series;
pub mod stats;

pub fn ldexp(x: u32, exp: u32) -> u32 {
    x * 2u32.pow(exp)
}

pub fn epsilon() -> f64 {
    1. * 2f64.powi(1 - (0f64.digits() as i32))
}

#[cfg(test)]
mod tests {
    use crate::{stats::erf, fraction::upper_gamma_fraction, epsilon};

    #[test]
    fn it_works() {
        dbg!(erf(5., false));

        let a = 5.5;
        let z = 3.;
        
        let f =upper_gamma_fraction(a, z, epsilon()) ;
        dbg!(z.powf(a) * (-z).exp() * f);

    }
}
