use core::ops::{Shl, Shr};

use super::Fixed32;

impl<const FRACTIONAL: usize> Shl<u32> for Fixed32<FRACTIONAL> {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self::Output {
        Self::Output {
            inner: self.inner << rhs
        }
    }
}
impl<const FRACTIONAL: usize> Shl<usize> for Fixed32<FRACTIONAL> {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        Self::Output {
            inner: self.inner << rhs
        }
    }
}

impl<const FRACTIONAL: usize> Shr<u32> for Fixed32<FRACTIONAL> {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        Self::Output {
            inner: self.inner >> rhs
        }
    }
}
impl<const FRACTIONAL: usize> Shr<usize> for Fixed32<FRACTIONAL> {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        Self::Output {
            inner: self.inner >> rhs
        }
    }
}