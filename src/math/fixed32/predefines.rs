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
}