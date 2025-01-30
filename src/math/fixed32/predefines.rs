use core::{f32::consts::PI, i32};

use super::Fixed32;

impl Fixed32 {
    pub const fn zero() -> Self {
        Self::from_i32(0)
    }
    pub const fn one() -> Self {
        Self::from_i32(1)
    }
    pub const fn neg_one() -> Self {
        Self::from_i32(-1)
    }
    pub const fn half_one() -> Self {
        Self::from_f32(0.5)
    }
    pub const fn pi() -> Self {
        Self::from_f32(PI)
    }
    pub const fn max_val() -> Self {
        Self {
            inner: i32::MAX
        }
    }
    pub const fn min_val() -> Self {
        Self {
            inner: i32::MIN
        }
    }
}