use crate::{math::{ray::Ray, types::FixFlt, vec3::Vec3}, tracer::objects::HitRecord};

use super::Scatterable;

pub struct MetalMat {
    pub albedo: Vec3 // color
}

impl Scatterable for MetalMat {
    fn scatter(&self, r: &Ray, rng: &mut FixFlt, hitrec: &HitRecord) -> (Ray, Vec3) {
        let mut direction = hitrec.normal + Vec3::random_unit_vec(rng);

        if direction.near_zero() {
            direction = hitrec.normal;
        }

        return (
            Ray::new(hitrec.point, direction),
            self.albedo
        );
    }
}
