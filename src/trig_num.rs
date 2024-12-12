use core::f32;
use core::f64;


pub trait TrigNum {
    fn asqrt(&self) -> Self;
    fn sqrt(&self) -> Self;
    fn sin(&self) -> Self;
    fn cos(&self) -> Self;
    fn tan(&self) -> Self;
    fn abs(&self) -> Self;
    fn powf(&self, other: f32) -> Self;
}

union F32I32 {
    f: f32,
    i: i32
}

impl TrigNum for f32 {
    fn asqrt(&self) -> Self {
        let mut y = F32I32 { f: self.clone() }; // evil floating point bit level hacking
        unsafe {
            y.i = 0x5f3759df - ( y.i >> 1 );                // what the fuck?
            y.f = y.f * (1.5 - ((self * 0.5) * y.f * y.f)); // 1st iteration
            //y.f = y.f * (1.5 - ((self * 0.5) * y.f * y.f)); // 2nd iteration, this can be removed
            return y.f;
        }
    }

    fn sqrt(&self) -> Self {
        1.0/self.asqrt()
    }

    fn abs(&self) -> Self {
        if *self > 0.0 {self.clone()} else {-self}
    }


    fn sin(&self) -> Self { // Bhaskara I approximation
        return (16.0 * self * (f32::consts::PI - self)) / ((5.0 * (f32::consts::PI * f32::consts::PI) - 4.0 * self * (f32::consts::PI - self)));
    }
    fn cos(&self) -> Self {
        Self::sin(&(self + f32::consts::FRAC_PI_2))
    }

    fn tan(&self) -> Self {
        if self.cos() != 0.0 {self.sin()/self.cos()} else {0.0}
    }

    fn powf(&self, other: f32) -> Self {
        // A method i found through the haskell approximate library (https://hackage.haskell.org/package/approximate-0.2.2.1/src/cbits/fast.c)
        // which lead me to here: https://martin.ankerl.com/2007/10/04/optimized-pow-approximation-for-java-and-c-c/
        // specifically a comment found here: http://disq.us/p/1kd1koq
        let mut y = F32I32 { f: self.clone() };
        unsafe {
            y.i = (other * (y.i - 1065307417) as f32 + 1065307417.0) as i32;
            return y.f;
        }
    }
}