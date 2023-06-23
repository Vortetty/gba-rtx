use agb_fixnum::{Num, num, FixedWidthUnsignedInteger, FixedWidthSignedInteger};

pub trait TrigFixedNum<const N: usize> {
    fn tan(self) -> Self;
    fn sqrt(self) -> Self;
}

impl<const N: usize> TrigFixedNum<N> for Num<i64, N> {
    fn tan(self) -> Self {
        //if Num::cos(self) < num!(1e-8) { return self; }
        return Num::sin(self) / Num::cos(self);
    }

    fn sqrt(self) -> Self {
        assert_eq!(N % 2, 0, "N must be even to be able to square root");
        assert!(self.to_raw() >= 0, "sqrt is only valid for positive numbers");
        let mut d = 1 << 30;
        let mut x = self.to_raw();
        let mut c = 0;

        while d > self.to_raw() {
            d >>= 2;
        }

        while d != 0 {
            if x >= c + d {
                x -= c + d;
                c = (c >> 1) + d;
            } else {
                c >>= 1;
            }
            d >>= 2;
        }
        Num::<i64, N>::new(c << (N / 2))
    }
}
