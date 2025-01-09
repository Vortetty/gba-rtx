use core::fmt::Debug;

use super::Fixed32;

impl<const FRACTIONAL: usize> Debug for Fixed32<FRACTIONAL> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Fixed32<FRACTIONAL: {}> {{ inner: {};{} }} (value: {})", FRACTIONAL, self.inner >> FRACTIONAL, ((self.inner >> FRACTIONAL) << FRACTIONAL) ^ self.inner, self.as_f32())
    }
}