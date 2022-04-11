mod digits;
pub use digits::Digits;
use num::{One, FromPrimitive, traits::real::Real};

pub mod fraction;

pub mod series;
pub mod stats;

pub fn ldexp(x: u32, exp: u32) -> u32 {
    x * 2u32.pow(exp)
}

pub fn epsilon<T: One + FromPrimitive + Real>() -> T {
    T::one() * T::from_u8(2).unwrap().powi(1 - (0f64.digits() as i32))
}

#[cfg(test)]
mod tests {
    use crate::{stats::{erf, Normal}, fraction::upper_gamma_fraction, epsilon};

    #[test]
    fn it_works() {
        dbg!(erf(5.));

        let a = 5.5;
        let z = 3.;
        
        let f =upper_gamma_fraction(a, z, epsilon()) ;
        dbg!(z.powf(a) * (-z).exp() * f);

        dbg!(Normal::standard().cdf(&0.2));

    }
}
