use core::fmt::Debug;

use super::Fixed32;
use crate::math::types::FRACTIONAL;

impl Debug for Fixed32 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Fixed32<FRACTIONAL: {}> {{ inner: {};{} }} (value: {})", FRACTIONAL, self.inner >> FRACTIONAL, ((self.inner >> FRACTIONAL) << FRACTIONAL) ^ self.inner, self.as_f32())
    }
}