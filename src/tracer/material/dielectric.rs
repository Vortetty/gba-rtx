use agb::println;

use crate::{math::{ray::Ray, types::FixFlt, vec3::{self, Vec3}}, tracer::objects::HitRecord};

use super::Scatterable;

#[derive(Clone, Copy)]
pub struct DielectricMat {
    pub albedo: Vec3, // color
    pub refraction: FixFlt,
    pub refraction_recip: FixFlt
}

impl Default for DielectricMat {
    fn default() -> Self {
        Self {
            albedo: Vec3::new(FixFlt::one(), FixFlt::one(), FixFlt::one()),
            refraction: FixFlt::zero(),
            refraction_recip: FixFlt::zero()
        }
    }
}

impl Scatterable for DielectricMat {
    fn scatter(&self, r: &Ray, rng: &mut FixFlt, hitrec: &HitRecord) -> (Ray, Vec3, bool) {
        // FROM OPENGL 4
        //
        // Parameters
        //
        // I
        //
        // Specifies the incident vector.
        // N
        //
        // Specifies the normal vector.
        // eta
        //
        // Specifies the ratio of indices of refraction.
        //
        // Description
        //
        // For a given incident vector I, surface normal N and ratio of indices of refraction, eta, refract returns the refraction vector, R.
        //
        // R is calculated as:
        //
        // k = 1.0 - eta * eta * (1.0 - dot(N, I) * dot(N, I));
        // if (k < 0.0)
        //     R = genType(0.0);       // or genDType(0.0)
        // else
        //     R = eta * I - (eta * dot(N, I) + sqrt(k)) * N;
        //
        // The input parameters I and N should be normalized in order to achieve the desired result.

        let I = r.direction.clone().unit_vec();
        let N = hitrec.normal.clone().unit_vec();
        let eta = if hitrec.front_face {
            FixFlt::from_f32(0.66666666666)
        } else {
            FixFlt::from_f32(1.5)
        };

        let IdotN = Vec3::dot_prod(&N, &I);
        let k = FixFlt::one() - ((eta*eta) * (1.0 - (IdotN*IdotN)));
        let refracted = if (k < FixFlt::zero()) {
            I.reflect(&hitrec.normal)
        } else {
            I*eta - N*(eta * IdotN + k.sqrt01())
        };

        // continue debug with https://www.shadertoy.com/view/7tBXDh

        return (
            Ray::new(hitrec.point, refracted),
            self.albedo,//if k < FixFlt::zero() { Vec3::new(-k, FixFlt::zero(), FixFlt::zero()) } else { Vec3::new(FixFlt::zero(), FixFlt::zero(), k) },
            false
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