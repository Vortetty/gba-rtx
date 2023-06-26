use crate::{vec3::Vec3, hittable::{Hittable, HitRecord}, ray::Ray, trig_num::trig_num};
use agb::timer::Timer;
use fixed::types::I14F18;

pub struct Sphere {
    pub center: Vec3,
    pub radius: I14F18
}

impl Hittable for Sphere {
    fn hit(&self, timer: &Timer, ray: &Ray, trace_min_len: I14F18, trace_max_len: I14F18, rec: &mut HitRecord) -> bool {
        let oc = ray.orig - self.center;
        let a = ray.dir.length_squared();
        let b_half = oc.dot_prod(ray.dir);
        let c = oc.length_squared() - self.radius*self.radius;
        let disc = b_half*b_half - a*c;

        if disc < I14F18::from_num(0.0) {
            return false;
        }
        let sqrtd = disc.sqrt();
        let mut root = (-b_half - sqrtd) / a;
        if (root < trace_min_len || trace_max_len < root) {
            root = (-b_half + sqrtd) / a;
            if (root < trace_min_len || trace_max_len < root) {
                return false;
            };
        };

        rec.trace_len = root;
        rec.point = ray.at(rec.trace_len);
        rec.normal = (rec.point - self.center) / self.radius;

        return true;
    }
}
