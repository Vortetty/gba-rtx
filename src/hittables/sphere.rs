use crate::{vec3::Vec3, hittable::{Hittable, HitRecord}, ray::Ray, trig_num::TrigNum};
use agb::timer::Timer;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: i32
}

impl Hittable for Sphere {
    fn hit(&self, _timer: &Timer, ray: &Ray, trace_min_len: f32, trace_max_len: f32, rec: &mut HitRecord) -> bool {
        let oc = ray.orig - self.center;
        let mut a = ray.dir.length_squared();
        let b_half = oc.dot_prod(ray.dir);
        let c = oc.length_squared() - self.radius*self.radius;
        let disc = b_half*b_half - a*c;

        if disc < (0.0) {
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
        rec.material = self.material;

        return true;
    }
}
