// use crate::math::{ray::Ray, types::FixFlt, vec3::Vec3};
use agb::println;
use micromath::F32Ext;

use sm64_gba_math::{F32, vek::*};

#[link_section = ".text_iwram"]
#[inline]
pub fn hit_sphere(center: Vec3<F32>, radius: F32, r: Ray<F32>) -> F32 {
    let mut r = r.clone();
    // r.reset_cached();
    let mut ray_to_sphere = center - r.origin;
    let dir_length_squared = r.direction.magnitude_squared(); // Ensures intersection is accurate regardless of magnitude
    let projection_length = r.direction.dot(ray_to_sphere);
    let center_to_ray_distance_squared = ray_to_sphere.magnitude_squared() - radius * radius;
    let discriminant = projection_length * projection_length - dir_length_squared * center_to_ray_distance_squared;

    if discriminant < F32::zero() {
        -F32::one()
    } else {
        (-projection_length - discriminant.sqrt()) * dir_length_squared.recip()
    }
}
