use alloc::vec::Vec;
use super::{interval::Interval, objects::{sphere::Sphere, HitRecord}};

use crate::{get_render_config::RenderConfig, math::{ray::Ray, types::{FixFlt, FixFltOnce}, vec3::Vec3}};

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
    pub fn ray_color(&mut self, r: &mut Ray, rng: &mut FixFlt, conf: &RenderConfig) -> Vec3 {
        //let t = hit_sphere(Vec3::new(FixFlt::zero(), FixFlt::zero(), FixFlt::neg_one()), FixFlt::half_one(), *r);
        let mut color_stack: Vec<Vec3> = vec![];
        let mut ctr = 0;

        let mut current_ray: Ray = *r;

        loop {
            ctr += 1;
            if ctr > conf.max_depth {
                return Vec3::new(FixFlt::zero(), FixFlt::zero(), FixFlt::zero());
                break;
            }

            let mut hitrec = HitRecord {
                point: Vec3::new(FixFlt::zero(), FixFlt::zero(), FixFlt::zero()),
                normal: Vec3::new(FixFlt::zero(), FixFlt::zero(), FixFlt::zero()),
                dist: FixFlt::max_val(),
                front_face: false
            };
            if self.calc_hit(&mut current_ray, Interval::new(FixFlt::from_f32(0.001), FixFlt::max_val()), &mut hitrec) {
                let direction = hitrec.normal + Vec3::random_unit_vec(rng);
                current_ray = Ray::new(hitrec.point, direction);
                continue;
            }

            let unit_dir = current_ray.direction.unit_vec();
            let verticality = (unit_dir.y + 1.0) * 0.5;
            color_stack.push(SKY_BOTTOM_COLOR * (1.0-verticality) + SKY_TOP_COLOR*verticality);
            break;
        }

        let mut out_color = color_stack[0];
        out_color = out_color * (FixFlt::from_f32(1.0) >> (ctr-1) as usize);
        Vec3::new(
            out_color.x.sqrt(),
            out_color.y.sqrt(),
            out_color.z.sqrt()
        )
    }
}