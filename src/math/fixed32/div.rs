use core::ops::Div;

use super::Fixed32;
use crate::math::types::FRACTIONAL;

impl Div<Self> for Fixed32 {
    type Output = Self;

    #[link_section = ".iwram"]
    fn div(self, rhs: Self) -> Self::Output {
        //Self::Output {
        //    inner: (self.inner << FRACTIONAL).div_euclid(rhs.inner)
        //}
        self * rhs.recip()
    }
}

impl Div<f32> for Fixed32 {
    type Output = Self;

    #[link_section = ".iwram"]
    fn div(self, rhs: f32) -> Self::Output {
        //Self::Output {
        //    inner: (self.inner << FRACTIONAL).div_euclid(Self::Output::from(rhs).inner)
        //}
        self * Self::Output::from(rhs).recip()
    }
}
impl Div<i32> for Fixed32 {
    type Output = Self;

    #[link_section = ".iwram"]
    fn div(self, rhs: i32) -> Self::Output {
        //Self::Output {
        //    inner: (self.inner << FRACTIONAL).div_euclid(rhs << FRACTIONAL)
        //}
        self * (Self::Output { inner: rhs << FRACTIONAL }).recip()
    }
}

impl Div<Fixed32> for f32 {
    type Output = Fixed32;

    #[link_section = ".iwram"]
    fn div(self, rhs: Fixed32) -> Self::Output {
        //Self::Output {
        //    inner: (Self::Output::from(self).inner << FRACTIONAL).div_euclid(rhs.inner)
        //}
        Self::Output::from(self) * rhs.recip()
    }
}
impl Div<Fixed32> for i32 {
    type Output = Fixed32;

    #[link_section = ".iwram"]
    fn div(self, rhs: Fixed32) -> Self::Output {
        //Self::Output {
        //    inner: (self << FRACTIONAL << FRACTIONAL).div_euclid(rhs.inner)
        //}
        (Self::Output { inner: self << FRACTIONAL }) * rhs.recip()
    }
}