use agb_fixnum::{Num, num, FixedWidthUnsignedInteger};

pub trait TrigFixedNum<const N: usize> {
    fn tan(self) -> Self;
}

impl<const N: usize> TrigFixedNum<N> for Num<i32, N> {
    fn tan(self) -> Self {
        //if Num::cos(self) < num!(1e-8) { return self; }
        return Num::sin(self) / Num::cos(self);
    }
}
