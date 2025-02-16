use crate::{math::{ray::Ray, types::FixFlt, vec3::Vec3}, tracer::objects::HitRecord};

use super::Scatterable;

#[derive(Clone, Copy)]
pub struct LambertianMat {
    pub albedo: Vec3 // color
}

impl Default for LambertianMat {
    fn default() -> Self {
        Self {
            albedo: Vec3::new(FixFlt::one(), FixFlt::one(), FixFlt::one()),
        }
    }
}

impl Scatterable for LambertianMat {
    fn scatter(&self, r: &Ray, rng: &mut FixFlt, hitrec: &HitRecord) -> (Ray, Vec3) {
        let mut direction = hitrec.normal + Vec3::random_unit_vec(rng);

        //if direction.near_zero() {
        //    direction = hitrec.normal;
        //}

        return (
            Ray::new(hitrec.point, direction),
            self.albedo
        );
    }
}
