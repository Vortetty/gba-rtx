use crate::math::types::FRACTIONAL;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fixed32 {
    inner: i32
}

impl Fixed32 {
    #[inline(always)]
    pub fn from(rhs: impl Into<f32>) -> Self {
        Self {
            inner: (rhs.into() * (1 << FRACTIONAL) as f32) as i32
        }
    }

    pub const fn from_f32(rhs: f32) -> Self {
        Self {
            inner: (rhs * (1 << FRACTIONAL) as f32) as i32
        }
    }
    pub const fn from_i32(rhs: i32) -> Self {
        Self {
            inner: rhs << FRACTIONAL
        }
    }

    #[inline(always)]
    pub const fn as_f32(&self) -> f32 {
        self.inner as f32 * (1.0 / (1 << FRACTIONAL) as f32)
    }
    #[inline(always)]
    pub const fn to_bits(&self) -> i32 {
        self.inner
    }
    #[inline(always)]
    pub const fn fractional(&self) -> usize {
        FRACTIONAL
    }

    #[inline(always)]
    pub fn abs(&self) -> Self {
        Self {
            inner: self.inner.abs()
        }
    }
}

mod add;
mod mul;
mod sub;
mod div;
mod debug;
mod shift;
mod predefines;
mod neg;
mod sqrt;
mod reciprocal;
mod anglemath;
mod ord;
