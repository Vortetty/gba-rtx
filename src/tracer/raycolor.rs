use agb::println;

use crate::math::{ray::Ray, types::FixFlt, vec3::{Color, Vec3}};

use super::sphere::hit_sphere;


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

pub fn ray_color(r: &mut Ray) -> Color {
    let t = hit_sphere(Vec3::new(FixFlt::zero(), FixFlt::zero(), FixFlt::neg_one()), FixFlt::half_one(), *r);
    if t > FixFlt::zero() {
        //return Color::new(1.0, 0.4, 0.55);
        let mut n = r.at(t);
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