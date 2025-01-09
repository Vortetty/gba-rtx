use agb::println;

use crate::math::{ray::Ray, vec3::{Color, Vec3}};

use super::sphere::hit_sphere;


const SKY_TOP_COLOR: Color = Color::new(
    0.459,
    0.478,
    0.749
);
const SKY_BOTTOM_COLOR: Color = Color::new(
    0.918,
    0.69,
    0.82
);

pub fn ray_color(r: &mut Ray) -> Color {
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, *r);
    if t > 0.0 {
        //return Color::new(1.0, 0.4, 0.55);
        let mut n = r.at(t);
        n.z += 1.0;
        n = n.unit_vec();
        let color = Color::new(
            n.x + 1.0,
            n.y + 1.0,
            n.z + 1.0
        ) * 0.5;

        return color;
    }

    let unit_dir = r.direction.unit_vec();
    let verticality = (unit_dir.y + 1.0) * 0.5;
    SKY_BOTTOM_COLOR * (1.0-verticality) + SKY_TOP_COLOR*verticality
}