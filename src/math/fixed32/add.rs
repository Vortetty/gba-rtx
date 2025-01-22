use core::ops::{Add, AddAssign};

use super::Fixed32;
use crate::math::types::FRACTIONAL;

impl Add<Self> for Fixed32 {
    type Output = Self;

    #[link_section = ".iwram"]
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            inner: self.inner + rhs.inner
        }
    }
}
impl AddAssign<Self> for Fixed32 {
    #[link_section = ".iwram"]
    fn add_assign(&mut self, rhs: Self) {
        self.inner += rhs.inner
    }
}

impl Add<f32> for Fixed32 {
    type Output = Self;

    #[link_section = ".iwram"]
    fn add(self, rhs: f32) -> Self::Output {
        Self::Output {
            inner: self.inner + Self::Output::from(rhs).inner
        }
    }
}
impl Add<i32> for Fixed32 {
    type Output = Self;

    #[link_section = ".iwram"]
    fn add(self, rhs: i32) -> Self::Output {
        Self::Output {
            inner: self.inner + (rhs << FRACTIONAL)
        }
    }
}
impl AddAssign<f32> for Fixed32 {
    #[link_section = ".iwram"]
    fn add_assign(&mut self, rhs: f32) {
        self.inner += Self::from(rhs).inner
    }
}
impl AddAssign<i32> for Fixed32 {
    #[link_section = ".iwram"]
    fn add_assign(&mut self, rhs: i32) {
        self.inner += (rhs << FRACTIONAL)
    }
}

impl Add<Fixed32> for f32 {
    type Output = Fixed32;

    #[link_section = ".iwram"]
    fn add(self, rhs: Fixed32) -> Self::Output {
        Self::Output {
            inner: rhs.inner + Self::Output::from(self).inner
        }
    }
}
impl Add<Fixed32> for i32 {
    type Output = Fixed32;

    #[link_section = ".iwram"]
    fn add(self, rhs: Fixed32) -> Self::Output {
        Self::Output {
            inner: (self << FRACTIONAL) + rhs.inner
        }
    }
}
