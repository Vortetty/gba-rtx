use agb::println;
use alloc::vec::Vec;
use super::{interval::Interval, objects::{sphere::Sphere, HitRecord}};

use crate::math::{ray::Ray, types::{FixFlt, FixFltOnce}, vec3::{Color, Vec3}};

pub struct Scene {
    pub spheres: Vec<Sphere>
}


const SKY_TOP_COLOR: Vec3 = Vec3::new(
    FixFlt::from_f32(0.459),
    FixFlt::from_f32(0.478),
    FixFlt::from_f32(0.749)
);
const SKY_BOTTOM_COLOR: Vec3 = Vec3::new(
    FixFlt::from_f32(0.918),
    FixFlt::from_f32(0.69),
    FixFlt::from_f32(0.82)
);

impl Scene {
    #[link_section = ".iwram"]
    fn calc_hit(&mut self, r: &mut Ray, ray_dist: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord {
            point: Vec3::new(FixFlt::zero(), FixFlt::zero(), FixFlt::zero()),
            normal: Vec3::new(FixFlt::zero(), FixFlt::zero(), FixFlt::zero()),
            dist: ray_dist.max,
            front_face: false
        };
        let mut has_hit = false;
        let mut closest = ray_dist;

        for sph in self.spheres.iter() {
            if sph.hit(r, closest, &mut temp_record) {
                has_hit = true;
                closest.max = temp_record.dist;
                rec.point = temp_record.point;
                rec.normal = temp_record.normal;
                rec.dist = temp_record.dist;
                rec.front_face = temp_record.front_face;
            }
        }

        return has_hit;
    }

    #[link_section = ".iwram"]
    pub fn ray_color(&mut self, r: &mut Ray, rng: &mut FixFlt) -> Vec3 {
        //let t = hit_sphere(Vec3::new(FixFlt::zero(), FixFlt::zero(), FixFlt::neg_one()), FixFlt::half_one(), *r);
        let mut color_stack: Vec<Vec3> = vec![];
        let mut ctr = 0;

        let mut current_ray: Ray = *r;

        loop {
            ctr += 1;
            if ctr > 8 {
                return Vec3::new(FixFlt::zero(), FixFlt::zero(), FixFlt::zero());
                break;
            }

            let mut hitrec = HitRecord {
                point: Vec3::new(FixFlt::zero(), FixFlt::zero(), FixFlt::zero()),
                normal: Vec3::new(FixFlt::zero(), FixFlt::zero(), FixFlt::zero()),
                dist: FixFlt::max_val(),
                front_face: false
            };
            if self.calc_hit(&mut current_ray, Interval::new(FixFlt::from_f32(0.005), FixFlt::max_val()), &mut hitrec) {
                //return Vec3::new(
                //    hitrec.normal.x + FixFlt::one(),
                //    hitrec.normal.y + FixFlt::one(),
                //    hitrec.normal.z + FixFlt::one()
                //) * FixFlt::half_one();
                let direction = hitrec.normal + Vec3::random_unit_vec(rng);
                //colorStack.push(self.ray_color(&mut Ray::new(hitrec.point, direction), rng) * FixFlt::half_one());
                current_ray = Ray::new(hitrec.point, direction);
                continue;
            }

            println!("r");
            let unit_dir = current_ray.direction.unit_vec();
            let verticality = (unit_dir.y + 1.0) * 0.5;
            color_stack.push(SKY_BOTTOM_COLOR * (1.0-verticality) + SKY_TOP_COLOR*verticality);
            break;
        }

        let mut out_color = color_stack[0];
        for _ in 0..(ctr-1) {
            out_color = out_color * FixFlt::from_f32(0.7);
        }
        out_color
    }
}