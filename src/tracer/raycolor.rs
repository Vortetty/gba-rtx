use agb::println;

// use crate::math::{ray::Ray, vec3::{Color, Vec3}};

use super::sphere::hit_sphere;

use sm64_gba_math::{F32, vek::*};

const SKY_TOP_COLOR: Vec3<F32> = Vec3::new(
    F32::from_f32(0.459),
    F32::from_f32(0.478),
    F32::from_f32(0.749)
);
const SKY_BOTTOM_COLOR: Vec3<F32> = Vec3::new(
    F32::from_f32(0.918),
    F32::from_f32(0.69),
    F32::from_f32(0.82)
);

#[link_section = ".text_iwram"]
#[inline]
pub fn ray_color(r: &mut Ray<F32>) -> Vec3<F32> {
    let t = hit_sphere(Vec3::new(F32::zero(), F32::zero(), -F32::one()), F32::from_f32(0.5), *r);
    if t > F32::zero() {
        //return Color::new(1.0, 0.4, 0.55);
        let mut n = r.origin + r.direction * t;
        n.z += F32::one();
        n = n.normalized();
        let color = (n + F32::one()) / F32::from_int(2);

        return color;
    }

    let unit_dir = r.direction.normalized();
    let verticality = (unit_dir.y + F32::one()).shr(1);
    SKY_BOTTOM_COLOR * (F32::one() - verticality) + SKY_TOP_COLOR*verticality
}
