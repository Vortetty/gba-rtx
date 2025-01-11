use crate::math::{ray::Ray, types::FixFlt, vec3::Vec3};
use agb::println;

use super::HitRecord;

pub struct Sphere {
    center: Vec3,
    radius: FixFlt
}

#[link_section = ".iwram"]
impl Sphere {
    pub fn hit(&self, r: &Ray, dist_min: FixFlt, dist_max: FixFlt, hitrec: &mut HitRecord) -> bool {
        let mut r = r.clone();
        r.reset_cached();
        let mut ray_to_sphere = self.center - r.origin;
        let dir_length_squared = r.direction.length_squared(); // Ensures intersection is accurate regardless of magnitude
        let projection_length = r.direction.dot_prod(&ray_to_sphere);
        let center_to_ray_distance_squared = ray_to_sphere.length_squared() - self.radius * self.radius;
        let discriminant = projection_length * projection_length - dir_length_squared * center_to_ray_distance_squared;

        if discriminant < FixFlt::zero() {
            return false;
        }

        let discriminant_sqrt = discriminant.sqrt();
        let mut root = (projection_length - discriminant_sqrt) / dir_length_squared;
        if root <= dist_min || dist_max <= root {
            root = (projection_length + discriminant_sqrt) / dir_length_squared;
            if root <= dist_min || dist_max <= root {
                return false;
            }
        }

        hitrec.dist = root;
        hitrec.point = r.at(root);
        let outward_normal = (hitrec.point - self.center) / self.radius;
        hitrec.set_face_normal(&r, &outward_normal);
        return true;
    }
}