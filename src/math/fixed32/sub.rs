use core::ops::{Sub, SubAssign};

use super::Fixed32;

impl<const FRACTIONAL: usize> Sub<Self> for Fixed32<FRACTIONAL> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            inner: self.inner - rhs.inner
        }
    }
}
impl<const FRACTIONAL: usize> SubAssign<Self> for Fixed32<FRACTIONAL> {
    fn sub_assign(&mut self, rhs: Self) {
        self.inner -= rhs.inner
    }
}

impl<const FRACTIONAL: usize> Sub<f32> for Fixed32<FRACTIONAL> {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self::Output {
            inner: self.inner - Self::Output::from(rhs).inner
        }
    }
}
impl<const FRACTIONAL: usize> Sub<i32> for Fixed32<FRACTIONAL> {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        Self::Output {
            inner: self.inner - (rhs << FRACTIONAL)
        }
    }
}
impl<const FRACTIONAL: usize> SubAssign<f32> for Fixed32<FRACTIONAL> {
    fn sub_assign(&mut self, rhs: f32) {
        self.inner -= Self::from(rhs).inner
    }
}
impl<const FRACTIONAL: usize> SubAssign<i32> for Fixed32<FRACTIONAL> {
    fn sub_assign(&mut self, rhs: i32) {
        self.inner -= rhs << FRACTIONAL
    }
}

impl<const FRACTIONAL: usize> Sub<Fixed32<FRACTIONAL>> for f32 {
    type Output = Fixed32<FRACTIONAL>;

    fn sub(self, rhs: Fixed32<FRACTIONAL>) -> Self::Output {
        Self::Output {
            inner: Self::Output::from(self).inner - rhs.inner
        }
    }
}
impl<const FRACTIONAL: usize> Sub<Fixed32<FRACTIONAL>> for i32 {
    type Output = Fixed32<FRACTIONAL>;

    fn sub(self, rhs: Fixed32<FRACTIONAL>) -> Self::Output {
        Self::Output {
            inner: (self << FRACTIONAL) - rhs.inner
        }
    }
}