use core::ops::Neg;

use super::Fixed32;
use crate::math::types::FRACTIONAL;

impl Neg for Fixed32 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            inner: -self.inner
        }
    }
}