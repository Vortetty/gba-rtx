use crate::math::{ray::Ray, types::FixFlt, vec3::Vec3};

use super::material::Material;

pub mod sphere;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub dist: FixFlt,
    pub front_face: bool,
    pub mat: Material
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction.dot_prod(outward_normal) < FixFlt::zero();
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        };
    }
}
