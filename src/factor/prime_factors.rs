use num::{FromPrimitive, Unsigned};

enum State<T> {
    Initial,
    Incomplete(T),
    Complete,
}

/// ```
/// use quickmaths::factor::PrimeFactors;
///
/// assert!(PrimeFactors::new(0u32).eq([]));
/// assert!(PrimeFactors::new(11u32).eq([11]));
/// assert!(PrimeFactors::new(25u32).eq([5, 5]));
/// assert!(PrimeFactors::new(33u32).eq([3, 11]));
/// assert!(PrimeFactors::new(2560u32).eq([2, 2, 2, 2, 2, 2, 2, 2, 2, 5]));
/// ```
pub struct PrimeFactors<T> {
    n: T,
    state: State<T>,
}

impl<T> PrimeFactors<T> {
    pub fn new(n: T) -> Self {
        Self {
            n,
            state: State::Initial,
        }
    }
}

impl<T> Iterator for PrimeFactors<T>
where
    T: Unsigned + FromPrimitive + PartialOrd + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.state {
                State::Initial => {
                    if self.n.is_zero() {
                        return None;
                    }
                    if self.n.is_one() {
                        self.state = State::Complete;
                        return Some(self.n);
                    } else {
                        self.state = State::Incomplete(T::from_u8(2).unwrap());
                    }
                }
                State::Incomplete(mut i) => {
                    if i * i <= self.n {
                        if !(self.n % i).is_zero() {
                            if i != T::from_u8(2).unwrap() {
                                i = i + T::one();
                            }
                            self.state = State::Incomplete(i + T::one());
                        } else {
                            self.n = self.n / i;
                            return Some(i);
                        }
                    } else {
                        self.state = State::Complete;
                        return Some(self.n);
                    }
                }
                State::Complete => {
                    return None;
                }
            }
        }
    }
}
