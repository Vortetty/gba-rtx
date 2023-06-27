use fixed::types::I34F30;

pub trait TrigNum {
    fn sqrt(&self) -> Self;
    fn sin(&self) -> Self;
    fn cos(&self) -> Self;
    fn tan(&self) -> Self;
    fn abs(&self) -> Self;
}

fn integer_sqrt(v: i64) -> i64 {
    let mut b = 1 << 30;
    let mut q = 0;
    let mut r = v;
    let mut t;
    while b > r {
        b >>= 2;
    }
    while b > 0 {
        t = q + b;
        q >>= 1;
        if r >= t {
            r -= t;
            q += b;
        }
        b >>= 2;
    }
    return q;
}

impl TrigNum for I34F30 {
    fn sqrt(&self) -> Self { // Works for any 32 bit fixed points
        //self.sqrt_iters(2) // Should be 10 for accuracy but this is a gameboy so... fuck it lol

        // ok so let's do dumb shiz and write a sqrt and just hope it works somehow
        //let mut t;
        //let mut q = 0;
        //let mut b = 0x40000000;
        //let mut r = u32::from_ne_bytes(self.to_ne_bytes());
        //while b > 0x40 {
        //    t = q + b;
        //    if r >= t {
        //        r -= t;
        //        q = t + b;
        //    }
        //    r <<= 1;
        //    b >>= 1;
        //}
        //q >>= 8;
        //return I34F30::from_ne_bytes(q.to_ne_bytes());

        // Ok so this is just as fast as the previous but supports more fp types soooo, we'll use this!
        return Self::from_ne_bytes((integer_sqrt(i64::from_ne_bytes(self.to_ne_bytes())) << (Self::FRAC_BITS >> 1) ).to_ne_bytes());
    }

    fn sin(&self) -> Self { // Bhaskara I approximation
        return (16 * self * (Self::PI - self)).checked_div((5 * (Self::PI * Self::PI) - 4 * self * (I34F30::PI - self))).ok_or_else(|| 0).unwrap();
    }
    fn cos(&self) -> Self {
        Self::sin(&(self + Self::FRAC_PI_2))
    }
    fn tan(&self) -> Self {
        Self::sin(self).checked_div(Self::cos(self)).ok_or_else(|| 0).unwrap()
    }

    fn abs(&self) -> Self {
        //if *self < Self::from_num(0) {
        //    self.clone()
        //} else {
        //    self.checked_neg().unwrap()
        //}
        return Self::from_ne_bytes(i64::from_ne_bytes(self.to_ne_bytes()).abs().to_ne_bytes());
    }
}