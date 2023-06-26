use agb::timer::Timer;
use agb_fixnum::{Num, num};
use fixed::types::I20F12;

use crate::{vec3::Vec3, rand::rand_double_range, trig_num::trig_num};

#[allow(dead_code)]
pub fn deg_to_rad(deg: I20F12) -> I20F12 {
    return deg * I20F12::from_num(3.14159265358979323846264338327950288) / I20F12::from_num(180.0);
}

pub fn random_in_unit_sphere(rng: &Timer) -> Vec3 {
    loop {
        let p = Vec3::rand_range(rng, I20F12::from_num(-1.0), I20F12::from_num(1.0));
        if p.length_squared() >= I20F12::from_num(1.0) {
            continue;
        };
        return p;
    }
}
pub fn random_in_unit_disk(rng: &Timer) -> Vec3 {
    loop {
        let p = Vec3::new(rand_double_range(rng, I20F12::from_num(-1.0), I20F12::from_num(1.0)), rand_double_range(rng, I20F12::from_num(-1.0), I20F12::from_num(1.0)), I20F12::from_num(0.0));
        if p.length_squared() >= I20F12::from_num(1.0) {
            continue;
        };
        return p;
    }
}
//fn random_unit_vector(rng: &Timer) -> Vec3 {
//    return random_in_unit_sphere(rng).unit_vector();
//}
pub fn random_in_hemisphere(normal: &Vec3, rng: &Timer) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere(rng);
    if in_unit_sphere.dot_prod(normal.clone()) > I20F12::from_num(0.0) {
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return (*v) - I20F12::from_num(2.0)*v.dot_prod(n.clone())*(*n);
}

pub fn refract(uv: &Vec3, normal: &Vec3, etai_over_etat: I20F12) -> Vec3 {
    let cos_theta = I20F12::min((-*uv).dot_prod(*normal), I20F12::from_num(1.0));
    let r_out_perp = etai_over_etat * (*uv + cos_theta*(*normal));
    let r_out_parallel = (((I20F12::from_num(1.0)-r_out_perp.length_squared()).abs().sqrt()).checked_neg().unwrap()) * (*normal);
    return r_out_perp + r_out_parallel;
}