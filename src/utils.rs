use agb::{timer::Timer, println};
use agb_fixnum::{Num, num};
use fixed::types::I34F30;

use crate::{vec3::Vec3, rand::rand_double_range, trig_num::TrigNum};

#[allow(dead_code)]
pub fn deg_to_rad(deg: I34F30) -> I34F30 {
    return deg * I34F30::from_num(3.14159265358979323846264338327950288) / I34F30::from_num(180.0);
}

pub fn random_in_unit_sphere(rng: &Timer) -> Vec3 {
    loop {
        let p = Vec3::rand_range(rng, I34F30::from_num(-1.0), I34F30::from_num(1.0));
        //println!("c1c {}, {}, {} ({})", p.x, p.y, p.z, p.length_squared());
        if p.length_squared() >= I34F30::from_num(1.0) {
            continue;
        };
        return p;
    }
}
pub fn random_in_unit_disk(rng: &Timer) -> Vec3 {
    loop {
        let p = Vec3::new(rand_double_range(rng, I34F30::from_num(-1.0), I34F30::from_num(1.0)), rand_double_range(rng, I34F30::from_num(-1.0), I34F30::from_num(1.0)), I34F30::from_num(0.0));
        if p.length_squared() >= I34F30::from_num(1.0) {
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
    if in_unit_sphere.dot_prod(normal.clone()) > I34F30::from_num(0.0) {
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return (*v) - I34F30::from_num(2.0)*v.dot_prod(n.clone())*(*n);
}

pub fn refract(uv: &Vec3, normal: &Vec3, etai_over_etat: I34F30) -> Vec3 {
    let cos_theta = I34F30::min((-*uv).dot_prod(*normal), I34F30::from_num(1.0));
    let r_out_perp = etai_over_etat * (*uv + cos_theta*(*normal));
    let r_out_parallel = (((I34F30::from_num(1.0)-r_out_perp.length_squared()).abs().sqrt()).checked_neg().unwrap()) * (*normal);
    return r_out_perp + r_out_parallel;
}