use super::{types::FixFlt, vec3::Vec3};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    #[inline(always)]
    pub fn at(&self, distance: FixFlt) -> Vec3 {
        self.origin + self.direction*distance
    }
}
