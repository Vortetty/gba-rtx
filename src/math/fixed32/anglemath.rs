use super::Fixed32;
use crate::math::types::FRACTIONAL;

impl Fixed32 {
    #[inline]
    pub fn deg_to_rad(&self) -> Self {
        *self * Self::pi() * Self::from_i32(180).const_recip()
    }
}
