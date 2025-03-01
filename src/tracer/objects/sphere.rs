use agb::println;

use crate::{math::{ray::Ray, types::FixFlt, vec3::Vec3}, tracer::material::Material};

use super::HitRecord;
use crate::tracer::interval::Interval;

pub struct Sphere {
    pub center: Vec3,
    pub radius: FixFlt,
    pub mat: Material
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            center: Vec3::default(),
            radius: FixFlt::zero(),
            mat: Material::default()
        }
    }
}

impl Sphere {
    pub fn hit(&self, r: &Ray, ray_dist: Interval, hitrec: &mut HitRecord) -> bool {
        let mut r = r.clone();
        r.reset_cached();

        let mut ray_to_sphere = self.center - r.origin; // oc
        let dir_length_squared = r.direction.length_squared(); // a
        let projection_length = r.direction.dot_prod(&ray_to_sphere); // h
        let center_to_ray_distance_squared = ray_to_sphere.length_squared() - self.radius * self.radius; // c

        let tmp = dir_length_squared * center_to_ray_distance_squared;
        let projsq = projection_length * projection_length;
        let discriminant = projsq - if tmp > FixFlt::zero() && tmp <= projsq { tmp } else { return false; };

        if discriminant < FixFlt::zero() {
            return false;
        }

        let discriminant_sqrt = discriminant.sqrt();
        let mut root = (projection_length - discriminant_sqrt) / dir_length_squared;
        if !ray_dist.surrounds(root) {
            root = (projection_length + discriminant_sqrt) / dir_length_squared;
            if !ray_dist.surrounds(root) {
                return false;
            }
        }

        hitrec.dist = root;
        hitrec.point = r.at(root);
        let outward_normal = (hitrec.point - self.center) / self.radius;
        hitrec.set_face_normal(&r, &outward_normal);
        hitrec.mat = self.mat;
        return true;
    }
}