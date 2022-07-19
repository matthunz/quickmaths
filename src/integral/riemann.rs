use num::{traits::real::Real, FromPrimitive};

pub trait Area<T> {
    fn area<F: FnMut(T) -> T>(&self, a: T, dx: T, i: usize, f: F) -> T;
}

pub enum Rectangle {
    MidPoint,
    Left,
    Right,
}

impl<T> Area<T> for Rectangle
where
    T: Real + FromPrimitive,
{
    fn area<F: FnMut(T) -> T>(&self, a: T, dx: T, i: usize, mut f: F) -> T {
        let x = match self {
            Self::Left => a + T::from_usize(i).unwrap() * dx,
            Self::Right => a + dx + T::from_usize(i).unwrap() * dx,
            Self::MidPoint => a + (dx / T::from_u8(2).unwrap()) + T::from_usize(i).unwrap() * dx,
        };
        f(x) * dx
    }
}

pub struct Trapezoid;

impl<T> Area<T> for Trapezoid
where
    T: Real + FromPrimitive,
{
    fn area<F: FnMut(T) -> T>(&self, a: T, dx: T, i: usize, mut f: F) -> T {
        (T::one() / T::from_u8(2).unwrap())
            * (dx)
            * (f(a + T::from_usize(i).unwrap() * dx) + f(a + T::from_usize(i + 1).unwrap() * dx))
    }
}

/// ```
/// use quickmaths::integral::RiemannSum;
/// use approx::assert_relative_eq;
/// 
/// let mid = RiemannSum::midpoint(1., 3., 1_000, |x| x);
/// assert_relative_eq!(mid.sum::<f32>(), 4.);
///
/// let left = RiemannSum::left(1., 3., 1_000, |x| x);
/// assert_relative_eq!(left.sum::<f32>(), 3.9980006);
/// 
/// let right = RiemannSum::right(1., 3., 1_000, |x| x);
/// assert_relative_eq!(right.sum::<f32>(), 4.0020003);
/// ```
// TODO midpoint, left, right
pub struct RiemannSum<F, T, S> {
    f: F,
    a: T,
    n: usize,
    i: usize,
    dx: T,
    shape: S,
}

impl<F, T, S> RiemannSum<F, T, S>
where
    T: Real + FromPrimitive,
{
    pub fn new(a: T, b: T, n: usize, f: F, shape: S) -> Self {
        Self {
            f,
            a,
            n,
            i: 0,
            dx: (b - a) / T::from_usize(n).unwrap(),
            shape,
        }
    }
}

impl<F, T> RiemannSum<F, T, Rectangle>
where
    T: Real + FromPrimitive,
{
    pub fn left(a: T, b: T, n: usize, f: F) -> Self {
        Self::new(a, b, n, f, Rectangle::Left)
    }

    pub fn right(a: T, b: T, n: usize, f: F) -> Self {
        Self::new(a, b, n, f, Rectangle::Right)
    }

    pub fn midpoint(a: T, b: T, n: usize, f: F) -> Self {
        Self::new(a, b, n, f, Rectangle::MidPoint)
    }
}

impl<F, T> RiemannSum<F, T, Trapezoid>
where
    T: Real + FromPrimitive,
{
    pub fn trapezoid(a: T, b: T, n: usize, f: F) -> Self {
        Self::new(a, b, n, f, Trapezoid)
    }
}

impl<F, T, S> Iterator for RiemannSum<F, T, S>
where
    F: FnMut(T) -> T,
    T: Real + FromPrimitive,
    S: Area<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.n {
            let part = self.shape.area(self.a, self.dx, self.i, &mut self.f);
            self.i += 1;
            Some(part)
        } else {
            None
        }
    }
}
