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
