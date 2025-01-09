use core::ops::{Add, AddAssign};

use super::Fixed32;

impl<const FRACTIONAL: usize> Add<Self> for Fixed32<FRACTIONAL> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            inner: self.inner + rhs.inner
        }
    }
}
impl<const FRACTIONAL: usize> AddAssign<Self> for Fixed32<FRACTIONAL> {
    fn add_assign(&mut self, rhs: Self) {
        self.inner += rhs.inner
    }
}

impl<const FRACTIONAL: usize> Add<f32> for Fixed32<FRACTIONAL> {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self::Output {
            inner: self.inner + Self::Output::from(rhs).inner
        }
    }
}
impl<const FRACTIONAL: usize> Add<i32> for Fixed32<FRACTIONAL> {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Self::Output {
            inner: self.inner + (rhs << FRACTIONAL)
        }
    }
}
impl<const FRACTIONAL: usize> AddAssign<f32> for Fixed32<FRACTIONAL> {
    fn add_assign(&mut self, rhs: f32) {
        self.inner += Self::from(rhs).inner
    }
}
impl<const FRACTIONAL: usize> AddAssign<i32> for Fixed32<FRACTIONAL> {
    fn add_assign(&mut self, rhs: i32) {
        self.inner += rhs << FRACTIONAL
    }
}

impl<const FRACTIONAL: usize> Add<Fixed32<FRACTIONAL>> for f32 {
    type Output = Fixed32<FRACTIONAL>;

    fn add(self, rhs: Fixed32<FRACTIONAL>) -> Self::Output {
        Self::Output {
            inner: rhs.inner + Self::Output::from(self).inner
        }
    }
}
impl<const FRACTIONAL: usize> Add<Fixed32<FRACTIONAL>> for i32 {
    type Output = Fixed32<FRACTIONAL>;

    fn add(self, rhs: Fixed32<FRACTIONAL>) -> Self::Output {
        Self::Output {
            inner: (self << FRACTIONAL) + rhs.inner
        }
    }
}
