use std::marker::PhantomData;

use num::{traits::real::Real, FromPrimitive};

pub struct Rectangle;

pub struct Trapezoid;

pub trait Shape<T> {
    fn shape<F: FnMut(T) -> T>(a: T, dx: T, i: usize, f: F) -> T;
}

impl<T> Shape<T> for Rectangle
where
    T: Real + FromPrimitive,
{
    fn shape<F: FnMut(T) -> T>(a: T, dx: T, i: usize, mut f: F) -> T {
        f(a + T::from_usize(i).unwrap() * dx) * dx
    }
}

impl<T> Shape<T> for Trapezoid
where
    T: Real + FromPrimitive,
{
    fn shape<F: FnMut(T) -> T>(a: T, dx: T, i: usize, mut f: F) -> T {
        (T::one() / T::from_u8(2).unwrap())
            * (dx)
            * (f(a + T::from_usize(i).unwrap() * dx) + f(a + T::from_usize(i + 1).unwrap() * dx))
    }
}

// TODO midpoint, left, right
pub struct RiemannSum<F, T, S> {
    f: F,
    a: T,
    n: usize,
    i: usize,
    dx: T,
    shape: PhantomData<S>,
}

impl<F, T, S> RiemannSum<F, T, S>
where
    T: Real + FromPrimitive,
{
    pub fn new(a: T, b: T, n: usize, f: F) -> Self {
        Self {
            f,
            a,
            n,
            i: 0,
            dx: (b - a) / T::from_usize(n).unwrap(),
            shape: PhantomData,
        }
    }
}

impl<F, T> RiemannSum<F, T, Rectangle>
where
    T: Real + FromPrimitive,
{
    pub fn rectangle(a: T, b: T, n: usize, f: F) -> Self {
        Self::new(a, b, n, f)
    }
}

impl<F, T> RiemannSum<F, T, Trapezoid>
where
    T: Real + FromPrimitive,
{
    pub fn trapezoid(a: T, b: T, n: usize, f: F) -> Self {
        Self::new(a, b, n, f)
    }
}

impl<F, T, S> Iterator for RiemannSum<F, T, S>
where
    F: FnMut(T) -> T,
    T: Real + FromPrimitive,
    S: Shape<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.n {
            let part = S::shape(self.a, self.dx, self.i, &mut self.f);
            Some(part)
        } else {
            None
        }
    }
}
