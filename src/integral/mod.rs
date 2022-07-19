//! Integral approximations

pub mod riemann;
use num::{traits::real::Real, FromPrimitive};
use rand::{distributions::Standard, prelude::Distribution};
pub use riemann::RiemannSum;

/// ```
/// use quickmaths::integral::monte_carlo_integration;
/// use approx::assert_relative_eq;
///
/// // Integrate a polynomial
/// let f = |x: f64| 3. * x.powi(2);
/// let result = monte_carlo_integration(0., 1., 100_000, f);
///
/// assert_relative_eq!(result, 1., epsilon = 1e-2);
/// ```
pub fn monte_carlo_integration<F, T>(a: T, b: T, n: usize, mut f: F) -> T
where
    Standard: Distribution<T>,
    F: FnMut(T) -> T,
    T: Real + FromPrimitive,
{
    let dx = (b - a) / T::from_usize(n).unwrap();
    let sum = (1..n).fold(T::zero(), |sum, _| {
        let x = a + (b - a) * rand::random();
        sum + f(x)
    });

    dx * sum
}

/// Calculate an approximation of the definite integral for function `f` from `start` to `end` with `n` steps
/// using [Simpson's rule](https://en.wikipedia.org/wiki/Simpson%27s_rule).
///
/// ```
/// use quickmaths::integral::simpson_integration;
/// use approx::assert_relative_eq;
///
/// // Calculate the length of the curve f(x) = x^2 for -5 <= x <= 5
/// // We should integrate sqrt(1 + (f'(x))^2)
/// let f = |x: f64| (1.0 + 4.0 * x * x).sqrt();
/// let result = simpson_integration(-5.0, 5.0, 1_000, f);
///    
/// let integral = |x: f64| (x * f(x) / 2.0) + ((2.0 * x).asinh() / 4.0);
/// assert_relative_eq!(result, integral(5.0) - integral(-5.0), epsilon = 1e-9);
/// ```
///
/// ```
/// use quickmaths::integral::simpson_integration;
/// use core::f64::consts::PI;
/// use approx::assert_relative_eq;
///
/// // Calculate area under f(x) = cos(x) + 5 for -pi <= x <= pi
/// // cosine should cancel out and the answer should be 2pi * 5
/// let f = |x: f64| x.cos() + 5.;
/// let result = simpson_integration(-PI, PI, 1_000, f);
///
/// assert_relative_eq!(result, 2.0 * PI * 5.0, epsilon = 1e-9);
/// ```
pub fn simpson_integration<F, T>(start: T, end: T, steps: usize, mut f: F) -> T
where
    F: FnMut(T) -> T,
    T: Real + FromPrimitive,
{
    let mut result = f(start) + f(end);
    let step = (end - start) / T::from_usize(steps).unwrap();

    for i in 1..steps {
        let x = start + step * T::from_usize(i).unwrap();
        result = if i % 2 == 0 {
            result + f(x) * T::from_u8(2).unwrap()
        } else {
            result + f(x) * T::from_u8(4).unwrap()
        };
    }

    result * (step / T::from_u8(3).unwrap())
}
