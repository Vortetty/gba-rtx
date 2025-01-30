use super::Fixed32;

impl Fixed32 {
    pub fn deg_to_rad(&self) -> Self {
        *self * Self::pi() * Self::from_i32(180).const_recip()
    }
}
