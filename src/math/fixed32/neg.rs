use core::ops::Neg;

use super::Fixed32;

impl Neg for Fixed32 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            inner: -self.inner
        }
    }
}