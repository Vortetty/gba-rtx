use agb::timer::Timer;

use crate::{ray::Ray, color::Color, hittable::HitRecord, vec3::Vec3};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray, rng: &Timer) -> bool;
    #[allow(unused_variables)]
    fn emitted(&self, u: f32, v: f32, p: &Vec3) -> Color {
        return Color::new(0.0, 0.0, 0.0);
    }
}