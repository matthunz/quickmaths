//! Integral approximations

pub mod riemann;
use num::{traits::real::Real, Float, FromPrimitive};
use rand::{distributions::Standard, prelude::Distribution};
pub use riemann::RiemannSum;

use self::riemann::Area;

pub struct Integral<F, T> {
    start: T,
    end: T,
    steps: usize,
    f: F,
}

impl<F, T> Integral<F, T> {
    pub fn new(start: T, end: T, steps: usize, f: F) -> Self {
        Self {
            start,
            end,
            steps,
            f,
        }
    }

    /// ```
    /// use quickmaths::Integral;
    /// use quickmaths::integral::riemann::Trapezoid;
    /// use approx::assert_relative_eq;
    ///
    /// let riemann = Integral::new(1., 3., 1_000, |x| x)
    ///     .riemann_sum(Trapezoid);
    /// assert_relative_eq!(riemann.sum::<f32>(), 4.);
    /// ```
    pub fn riemann_sum<A>(self, area: A) -> RiemannSum<F, T, A>
    where
        A: Area<T>,
        T: Float + FromPrimitive,
    {
        RiemannSum::new(self.start, self.end, self.steps, self.f, area)
    }

    /// ```
    /// use quickmaths::Integral;
    /// use approx::assert_relative_eq;
    ///
    /// // Integrate a polynomial
    /// let f = |x: f64| 3. * x.powi(2);
    /// let mut integral = Integral::new(0., 1., 100_000, f);
    ///
    /// assert_relative_eq!(integral.monte_carlo(), 1., epsilon = 1e-2);
    /// ```
    pub fn monte_carlo(&mut self) -> T
    where
        Standard: Distribution<T>,
        F: FnMut(T) -> T,
        T: Real + FromPrimitive,
    {
        let diff = self.end - self.start;
        let dx = diff / T::from_usize(self.steps).unwrap();
        let sum = (1..self.steps).fold(T::zero(), |sum, _| {
            let x = self.start + diff * rand::random();
            sum + (self.f)(x)
        });

        dx * sum
    }

    /// Calculate an approximation of the definite integral for function `f` from `start` to `end` with `n` steps
    /// using [Simpson's rule](https://en.wikipedia.org/wiki/Simpson%27s_rule).
    ///
    /// ```
    /// use quickmaths::Integral;
    /// use approx::assert_relative_eq;
    ///
    /// // Calculate the length of the curve f(x) = x^2 for -5 <= x <= 5
    /// // We should integrate sqrt(1 + (f'(x))^2)
    /// let f = |x: f64| (1.0 + 4.0 * x * x).sqrt();
    /// let mut integral = Integral::new(-5.0, 5.0, 1_000, f);
    ///    
    /// let f_integral = |x: f64| (x * f(x) / 2.0) + ((2.0 * x).asinh() / 4.0);
    /// assert_relative_eq!(integral.simpson(), f_integral(5.0) - f_integral(-5.0), epsilon = 1e-9);
    /// ```
    ///
    /// ```
    /// use quickmaths::Integral;
    /// use core::f64::consts::PI;
    /// use approx::assert_relative_eq;
    ///
    /// // Calculate area under f(x) = cos(x) + 5 for -pi <= x <= pi
    /// // cosine should cancel out and the answer should be 2pi * 5
    /// let f = |x: f64| x.cos() + 5.;
    /// let mut integral = Integral::new(-PI, PI, 1_000, f);
    ///
    /// assert_relative_eq!(integral.simpson(), 2.0 * PI * 5.0, epsilon = 1e-9);
    /// ```
    pub fn simpson(&mut self) -> T
    where
        F: FnMut(T) -> T,
        T: Real + FromPrimitive,
    {
        let mut result = (self.f)(self.start) + (self.f)(self.end);
        let step = (self.end - self.start) / T::from_usize(self.steps).unwrap();

        for i in 1..self.steps {
            let x = self.start + step * T::from_usize(i).unwrap();
            result = if i % 2 == 0 {
                result + (self.f)(x) * T::from_u8(2).unwrap()
            } else {
                result + (self.f)(x) * T::from_u8(4).unwrap()
            };
        }

        result * (step / T::from_u8(3).unwrap())
    }
}
