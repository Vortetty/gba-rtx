use crate::{vec3::Vec3, hittable::{Hittable, HitRecord}, ray::Ray, trig_num::TrigNum};
use agb::timer::Timer;
use fixed::types::I34F30;

pub struct Sphere {
    pub center: Vec3,
    pub radius: I34F30
}

impl Hittable for Sphere {
    fn hit(&self, timer: &Timer, ray: &Ray, trace_min_len: I34F30, trace_max_len: I34F30, rec: &mut HitRecord) -> bool {
        let oc = ray.orig - self.center;
        let mut a = ray.dir.length_squared();
        let b_half = oc.dot_prod(ray.dir);
        let c = oc.length_squared() - self.radius*self.radius;
        let disc = b_half*b_half - a*c;

        if a <= 0 {
            a = I34F30::from_num(0.001);
        }

        if disc < I34F30::from_num(0.0) {
            return false;
        }
        let sqrtd = disc.sqrt();
        let mut root = (-b_half - sqrtd) / a;
        if root < trace_min_len || trace_max_len < root {
            root = (-b_half + sqrtd) / a;
            if root < trace_min_len || trace_max_len < root {
                return false;
            };
        };

        rec.trace_len = root;
        rec.point = ray.at(rec.trace_len);
        rec.normal = (rec.point - self.center) / self.radius;
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);

        return true;
    }
}
