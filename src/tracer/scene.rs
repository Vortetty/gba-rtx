use alloc::vec::Vec;
use arrayvec::ArrayVec;
use super::{interval::Interval, material::{Material, MaterialManager}, objects::{sphere::Sphere, HitRecord}};

use crate::{get_render_config::RenderConfig, math::{ray::Ray, types::{FixFlt, FixFltOnce}, vec3::Vec3}};

pub struct Scene {
    pub spheres: Vec<Sphere>
}


//const SKY_TOP_COLOR: Vec3 = Vec3::new(
//    FixFlt::from_f32(0.459),
//    FixFlt::from_f32(0.478),
//    FixFlt::from_f32(0.749)
//);
//const SKY_BOTTOM_COLOR: Vec3 = Vec3::new(
//    FixFlt::from_f32(0.918),
//    FixFlt::from_f32(0.69),
//    FixFlt::from_f32(0.82)
//);

const SKY_TOP_COLOR: Vec3 = Vec3::new(
    FixFlt::from_f32(0.5),
    FixFlt::from_f32(0.7),
    FixFlt::from_f32(1.0)
);
const SKY_BOTTOM_COLOR: Vec3 = Vec3::new(
    FixFlt::from_f32(1.0),
    FixFlt::from_f32(1.0),
    FixFlt::from_f32(1.0)
);

impl Scene {
    #[link_section = ".iwram"]
    fn calc_hit(&mut self, r: &mut Ray, ray_dist: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord {
            point: Vec3::new(FixFlt::zero(), FixFlt::zero(), FixFlt::zero()),
            normal: Vec3::new(FixFlt::zero(), FixFlt::zero(), FixFlt::zero()),
            dist: ray_dist.max,
            front_face: false,
            mat: Material {
                mat_id: 0,
                mat_type: super::material::MaterialType::LAMBERTIAN
            },
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
                rec.mat = temp_record.mat;
            }
        }

        return has_hit;
    }

    #[link_section = ".iwram"]
    pub fn ray_color(&mut self, r: &mut Ray, rng: &mut FixFlt, conf: &RenderConfig, mat_mgr: &MaterialManager) -> Vec3 {
        //let t = hit_sphere(Vec3::new(FixFlt::zero(), FixFlt::zero(), FixFlt::neg_one()), FixFlt::half_one(), *r);
        let mut ctr = 0;

        let mut tmp_color: Vec3 = Vec3::new(FixFlt::zero(), FixFlt::zero(), FixFlt::zero());
        let mut out_color: Vec3 = Vec3::new(FixFlt::one(), FixFlt::one(), FixFlt::one());
        let mut current_ray: Ray = *r;

        loop {
            ctr += 1;
            if ctr > conf.max_depth {
                unsafe {
                    out_color = out_color * FixFlt::zero();
                }
                break;
            }

            let mut hitrec = HitRecord {
                point: Vec3::new(FixFlt::zero(), FixFlt::zero(), FixFlt::zero()),
                normal: Vec3::new(FixFlt::zero(), FixFlt::zero(), FixFlt::zero()),
                dist: FixFlt::max_val(),
                front_face: false,
                mat: Material {
                    mat_id: 0,
                    mat_type: super::material::MaterialType::LAMBERTIAN
                },
            };
            if self.calc_hit(&mut current_ray, Interval::new(FixFlt::from_f32(0.001), FixFlt::max_val()), &mut hitrec) {
                (current_ray, tmp_color) = mat_mgr.scatter(&hitrec.mat, r, rng, &hitrec);
                unsafe {
                    out_color = out_color * tmp_color;
                }
                continue;
            }

            let unit_dir = current_ray.direction.unit_vec();
            let verticality = (unit_dir.y + 1.0) * 0.5;
            unsafe {
                out_color = out_color * (SKY_BOTTOM_COLOR * (1.0-verticality) + SKY_TOP_COLOR*verticality);
            }
            break;
        }

        Vec3::new(
            out_color.x.sqrt01(),
            out_color.y.sqrt01(),
            out_color.z.sqrt01()
        )
    }
}