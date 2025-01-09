use crate::math::{ray::Ray, types::FixFlt, vec3::Vec3};
use agb::println;
use micromath::F32Ext;

pub fn hit_sphere(center: Vec3, radius: FixFlt, r: Ray) -> FixFlt {
    let mut r = r.clone();
    r.reset_cached();
    let mut ray_to_sphere = center - r.origin;
    let dir_length_squared = r.direction.length_squared(); // Ensures intersection is accurate regardless of magnitude
    let projection_length = r.direction.dot_prod(&ray_to_sphere);
    let center_to_ray_distance_squared = ray_to_sphere.length_squared() - radius * radius;
    let discriminant = projection_length * projection_length - dir_length_squared * center_to_ray_distance_squared;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-projection_length - discriminant.sqrt()) / dir_length_squared
    }
}