use agb::{timer::Timer};


use crate::{vec3::Vec3, rand::rand_double_range, trig_num::TrigNum};

#[allow(dead_code)]
pub fn deg_to_rad(deg: f32) -> f32 {
    return deg * (3.14159265358979323846264338327950288) / (180.0);
}

pub fn random_in_unit_sphere(rng: &Timer) -> Vec3 {
    loop {
        let p = Vec3::rand_range(rng, -1.0, 1.0);
        //println!("c1c {}, {}, {} ({})", p.x, p.y, p.z, p.length_squared());
        if p.length_squared() >= (1.0) {
            continue;
        };
        return p;
    }
}
pub fn random_in_unit_disk(rng: &Timer) -> Vec3 {
    loop {
        let p = Vec3::new(rand_double_range(rng, -1.0, 1.0), rand_double_range(rng, -1.0, 1.0), 0.0);
        if p.length_squared() >= (1.0) {
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
    if in_unit_sphere.dot_prod(normal.clone()) > (0.0) {
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return (*v) - (2.0)*v.dot_prod(n.clone())*(*n);
}

pub fn refract(uv: &Vec3, normal: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = f32::min((-*uv).dot_prod(*normal), 1.0);
    let r_out_perp = etai_over_etat * (*uv + cos_theta*(*normal));
    let r_out_parallel = (-(((1.0)-r_out_perp.length_squared()).abs().sqrt())) * (*normal);
    return r_out_perp + r_out_parallel;
}