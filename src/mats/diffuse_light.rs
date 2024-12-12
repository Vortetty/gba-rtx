use agb::timer::Timer;

use crate::{
    color::Color, hittable::HitRecord, material::Material, ray::Ray, utils::random_in_hemisphere,
};

#[derive(Clone, Copy)]
pub struct DiffuseLight {
    pub emit: Color,
}

impl Material for DiffuseLight {
    #[allow(unused_variables)]
    fn scatter(
        &self,
        ray_in: &crate::ray::Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut crate::ray::Ray,
        rng: &Timer,
    ) -> bool {
        return false;
    }

    #[allow(unused_variables)]
    fn emitted(&self, u: f32, v: f32, p: &crate::vec3::Vec3) -> Color {
        return self.emit;
    }
}
