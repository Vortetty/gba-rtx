use crate::{
    math::{ray::Ray, types::FixFlt, vec3::Vec3},
    tracer::objects::HitRecord,
};

use super::Scatterable;

#[derive(Clone, Copy)]
pub struct MetalMat {
    pub albedo: Vec3, // color
    pub matte: FixFlt,
}

impl Default for MetalMat {
    fn default() -> Self {
        Self {
            albedo: Vec3::new(FixFlt::one(), FixFlt::one(), FixFlt::one()),
            matte: FixFlt::zero(),
        }
    }
}

impl Scatterable for MetalMat {
    fn scatter(&self, r: &Ray, rng: &mut FixFlt, hitrec: &HitRecord) -> (Ray, Vec3) {
        return (
            Ray::new(
                hitrec.point,
                r.direction.reflect(&hitrec.normal).unit_vec()
                    + Vec3::random_unit_vec(rng) * self.matte,
            ),
            self.albedo,
        );
    }
}
