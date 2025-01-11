use core::ops::Mul;

use agb::println;

use super::Fixed32;
use crate::math::types::FRACTIONAL;

impl Mul<Self> for Fixed32 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            inner: (((self.inner as i64) * (rhs.inner as i64)) >> FRACTIONAL) as i32
        }
    }
}

impl Mul<f32> for Fixed32 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            inner: ((self.inner as i64) * (Self::Output::from(rhs).inner as i64) >> FRACTIONAL) as i32
        }
    }
}
impl Mul<i32> for Fixed32 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::Output {
            inner: self.inner * rhs
        }
    }
}

impl Mul<Fixed32> for f32 {
    type Output = Fixed32;

    fn mul(self, rhs: Fixed32) -> Self::Output {
        Self::Output {
            inner: ((Self::Output::from(self).inner as i64) * (rhs.inner as i64) >> FRACTIONAL) as i32
        }
    }
}
impl Mul<Fixed32> for i32 {
    type Output = Fixed32;

    fn mul(self, rhs: Fixed32) -> Self::Output {
        Self::Output {
            inner: self * rhs.inner
        }
    }
}

