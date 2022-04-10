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
    use crate::stats::erf;

    #[test]
    fn it_works() {
        dbg!(erf(0.2, false));
    }
}
