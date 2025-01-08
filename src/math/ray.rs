use super::{types::FixFlt, vec3::Vec3};

#[derive(Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    #[inline(always)]
    pub fn at(&self, distance: FixFlt) -> Vec3 {
        self.origin + self.direction*distance
    }

    #[inline(always)]
    pub const fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction
        }
    }
}
