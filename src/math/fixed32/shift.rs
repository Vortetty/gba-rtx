use core::ops::{Shl, Shr};

use super::Fixed32;

impl Shl<u32> for Fixed32 {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: u32) -> Self::Output {
        Self::Output {
            inner: self.inner << rhs
        }
    }
}
impl Shl<usize> for Fixed32 {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: usize) -> Self::Output {
        Self::Output {
            inner: self.inner << rhs
        }
    }
}

impl Shr<u32> for Fixed32 {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: u32) -> Self::Output {
        Self::Output {
            inner: self.inner >> rhs
        }
    }
}
impl Shr<usize> for Fixed32 {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: usize) -> Self::Output {
        Self::Output {
            inner: self.inner >> rhs
        }
    }
}