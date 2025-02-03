use crate::{
    math::{ray::Ray, types::FixFlt, vec3::Vec3},
    tracer::objects::HitRecord,
};

use super::Scatterable;

pub struct MetalMat {
    pub albedo: Vec3, // color
}

impl Scatterable for MetalMat {
    fn scatter(&self, r: &Ray, rng: &mut FixFlt, hitrec: &HitRecord) -> (Ray, Vec3) {
        return (
            Ray::new(hitrec.point, r.direction.reflect(&hitrec.normal)),
            self.albedo,
        );
    }
}
