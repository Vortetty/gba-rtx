use core::ops::Div;

use super::Fixed32;

impl<const FRACTIONAL: usize> Div<Self> for Fixed32<FRACTIONAL> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            inner: (self.inner << FRACTIONAL).div_euclid(rhs.inner)
        }
    }
}

impl<const FRACTIONAL: usize> Div<f32> for Fixed32<FRACTIONAL> {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            inner: (self.inner << FRACTIONAL).div_euclid(Self::Output::from(rhs).inner)
        }
    }
}
impl<const FRACTIONAL: usize> Div<i32> for Fixed32<FRACTIONAL> {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self::Output {
            inner: (self.inner << FRACTIONAL).div_euclid(rhs << FRACTIONAL)
        }
    }
}

impl<const FRACTIONAL: usize> Div<Fixed32<FRACTIONAL>> for f32 {
    type Output = Fixed32<FRACTIONAL>;

    fn div(self, rhs: Fixed32<FRACTIONAL>) -> Self::Output {
        Self::Output {
            inner: (Self::Output::from(self).inner << FRACTIONAL).div_euclid(rhs.inner)
        }
    }
}
impl<const FRACTIONAL: usize> Div<Fixed32<FRACTIONAL>> for i32 {
    type Output = Fixed32<FRACTIONAL>;

    fn div(self, rhs: Fixed32<FRACTIONAL>) -> Self::Output {
        Self::Output {
            inner: (self << FRACTIONAL << FRACTIONAL).div_euclid(rhs.inner)
        }
    }
}
