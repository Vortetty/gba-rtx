use core::ops::Neg;

use super::Fixed32;

impl<const FRACTIONAL: usize> Neg for Fixed32<FRACTIONAL> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            inner: -self.inner
        }
    }
}