use super::{types::FixFlt, vec3::Vec3};

#[derive(Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    
    pub fn at(&self, distance: FixFlt) -> Vec3 {
        self.origin + self.direction*distance
    }

    
    pub const fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction
        }
    }

    
    pub fn reset_cached(&mut self) {
        self.origin.reset_cached();
        self.direction.reset_cached();
    }
}
