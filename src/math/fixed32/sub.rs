use core::ops::{Sub, SubAssign};

use super::Fixed32;
use crate::math::types::FRACTIONAL;

impl Sub<Self> for Fixed32 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            inner: self.inner - rhs.inner
        }
    }
}
impl SubAssign<Self> for Fixed32 {
    fn sub_assign(&mut self, rhs: Self) {
        self.inner -= rhs.inner
    }
}

impl Sub<f32> for Fixed32 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self::Output {
            inner: self.inner - Self::Output::from(rhs).inner
        }
    }
}
impl Sub<i32> for Fixed32 {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        Self::Output {
            inner: self.inner - (rhs << FRACTIONAL)
        }
    }
}
impl SubAssign<f32> for Fixed32 {
    fn sub_assign(&mut self, rhs: f32) {
        self.inner -= Self::from(rhs).inner
    }
}
impl SubAssign<i32> for Fixed32 {
    fn sub_assign(&mut self, rhs: i32) {
        self.inner -= rhs << FRACTIONAL
    }
}

impl Sub<Fixed32> for f32 {
    type Output = Fixed32;

    fn sub(self, rhs: Fixed32) -> Self::Output {
        Self::Output {
            inner: Self::Output::from(self).inner - rhs.inner
        }
    }
}
impl Sub<Fixed32> for i32 {
    type Output = Fixed32;

    fn sub(self, rhs: Fixed32) -> Self::Output {
        Self::Output {
            inner: (self << FRACTIONAL) - rhs.inner
        }
    }
}
