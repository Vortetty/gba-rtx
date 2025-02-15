use crate::{math::{ray::Ray, types::FixFlt, vec3::Vec3}, tracer::objects::HitRecord};

use super::Scatterable;

pub struct DielectricMat {
    pub albedo: Vec3, // color
    pub refraction: FixFlt,
    pub refraction_recip: FixFlt
}

impl Scatterable for DielectricMat {
    fn scatter(&self, r: &Ray, rng: &mut FixFlt, hitrec: &HitRecord) -> (Ray, Vec3) {
        let ri = if hitrec.front_face { self.refraction_recip } else { self.refraction };
        let unit_dir = r.direction.clone().unit_vec();

        let cos_theta = (-unit_dir.dot_prod(&hitrec.normal)).min(FixFlt::one());
        let sin_theta = (FixFlt::one() - cos_theta*cos_theta).sqrt();

        let refracted = if (ri*sin_theta > FixFlt::one()) || reflectance(cos_theta, ri) > rng.next_rand_frac() {
            r.direction.reflect(&hitrec.normal)
        } else {
            unit_dir.refract(&hitrec.normal, ri, cos_theta)
        };

        return (
            Ray::new(hitrec.point, refracted),
            self.albedo
        );
    }
}

fn reflectance(cosine: FixFlt, refractive_index: FixFlt) -> FixFlt {
    let mut r0 = (FixFlt::one() - refractive_index) / (FixFlt::one() + refractive_index);
    r0 = r0 * r0;
    let one_sub_cos = FixFlt::one() - cosine;
    // r0 + (1-r0)*std::pow((1 - cosine),5)
    r0 + (FixFlt::one() - r0) * (one_sub_cos*one_sub_cos*one_sub_cos*one_sub_cos*one_sub_cos)
}