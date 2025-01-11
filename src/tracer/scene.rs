use agb::println;
use alloc::vec::Vec;
use super::objects::{sphere::Sphere, HitRecord};

use crate::math::{ray::Ray, types::FixFlt, vec3::{Color, Vec3}};

struct Scene {
    pub spheres: Vec<Sphere>
}


const SKY_TOP_COLOR: Color = Color::new(
    FixFlt::from_f32(0.459),
    FixFlt::from_f32(0.478),
    FixFlt::from_f32(0.749)
);
const SKY_BOTTOM_COLOR: Color = Color::new(
    FixFlt::from_f32(0.918),
    FixFlt::from_f32(0.69),
    FixFlt::from_f32(0.82)
);

impl Scene {
    fn calc_hit(&mut self, r: &mut Ray, dist_min: FixFlt, dist_max: FixFlt, rec: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord {
            point: Vec3::new(FixFlt::zero(), FixFlt::zero(), FixFlt::zero()),
            normal: Vec3::new(FixFlt::zero(), FixFlt::zero(), FixFlt::zero()),
            dist: dist_max,
            front_face: false
        };
        let mut has_hit = false;
        let mut closest = dist_max;

        for sph in self.spheres.iter() {
            if sph.hit(r, dist_min, closest, &mut temp_record) {
                has_hit = true;
                closest = temp_record.dist;
                rec.point = temp_record.point;
                rec.normal = temp_record.normal;
                rec.dist = temp_record.dist;
                rec.front_face = temp_record.front_face;
            }
        }

        return has_hit;
    }

    #[link_section = ".iwram"]
    pub fn ray_color(&mut self, r: &mut Ray, dist_min: FixFlt, dist_max: FixFlt) -> Color {
        let t = hit_sphere(Vec3::new(FixFlt::zero(), FixFlt::zero(), FixFlt::neg_one()), FixFlt::half_one(), *r);

        if t.is_some() {
            //return Color::new(1.0, 0.4, 0.55);
            let mut n = r.at(t.unwrap());
            n.z += FixFlt::one();
            n = n.unit_vec();
            let color = Color::new(
                n.x + FixFlt::one(),
                n.y + FixFlt::one(),
                n.z + FixFlt::one()
            ) * FixFlt::half_one();

            return color;
        }

        let unit_dir = r.direction.unit_vec();
        let verticality = (unit_dir.y + 1.0) * 0.5;
        SKY_BOTTOM_COLOR * (1.0-verticality) + SKY_TOP_COLOR*verticality
    }
}