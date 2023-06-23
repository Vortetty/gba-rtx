use agb_fixnum::Num;

use crate::vec3::Vec3;

#[derive(Clone)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3
}

impl Ray {
    pub fn at(&self, t: Num<i64, 20>) -> Vec3 {
        return self.orig + t*self.dir;
    }

    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        return Ray {
            orig: origin,
            dir: direction
        };
    }
}
