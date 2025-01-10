use core::ops::Mul;

use super::Fixed32;

impl<const FRACTIONAL: usize> Mul<Self> for Fixed32<FRACTIONAL> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            inner: (self.inner * rhs.inner) >> FRACTIONAL
        }
    }
}

impl<const FRACTIONAL: usize> Mul<f32> for Fixed32<FRACTIONAL> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            inner: (self.inner * Self::Output::from(rhs).inner) >> FRACTIONAL
        }
    }
}
impl<const FRACTIONAL: usize> Mul<i32> for Fixed32<FRACTIONAL> {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::Output {
            inner: self.inner * rhs
        }
    }
}

impl<const FRACTIONAL: usize> Mul<Fixed32<FRACTIONAL>> for f32 {
    type Output = Fixed32<FRACTIONAL>;

    fn mul(self, rhs: Fixed32<FRACTIONAL>) -> Self::Output {
        Self::Output {
            inner: (Self::Output::from(self).inner * rhs.inner) >> FRACTIONAL
        }
    }
}
impl<const FRACTIONAL: usize> Mul<Fixed32<FRACTIONAL>> for i32 {
    type Output = Fixed32<FRACTIONAL>;

    fn mul(self, rhs: Fixed32<FRACTIONAL>) -> Self::Output {
        Self::Output {
            inner: self * rhs.inner
        }
    }
}

