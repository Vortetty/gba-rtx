use core::{f32::consts::PI, i32};

use super::Fixed32;

impl<const FRACTIONAL: usize> Fixed32<FRACTIONAL> {
    #[inline]
    pub const fn zero() -> Self {
        Self::from_i32(0)
    }
    #[inline]
    pub const fn one() -> Self {
        Self::from_i32(1)
    }
    #[inline]
    pub const fn neg_one() -> Self {
        Self::from_i32(-1)
    }
    #[inline]
    pub const fn half_one() -> Self {
        Self::from_f32(0.5)
    }
    #[inline]
    pub const fn pi() -> Self {
        Self::from_f32(PI)
    }
    #[inline]
    pub const fn max_val() -> Self {
        Self {
            inner: i32::MAX
        }
    }
    #[inline]
    pub const fn min_val() -> Self {
        Self {
            inner: i32::MIN
        }
    }
}