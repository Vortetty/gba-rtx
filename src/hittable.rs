use agb::timer::Timer;
use fixed::types::I14F18;

use crate::{vec3::Vec3, ray::Ray};

struct HitRecord {
    p: Vec3,
    norm: Vec3,
    trace_len: I14F18
}

trait Hittable {
    fn hit(timer: &Timer, ray: &Ray, trace_min_len: I14F18, trace_max_len: I14F18, rec: HitRecord);
}