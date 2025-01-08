use crate::{math::{ray::Ray, types::FixFlt, vec3::{Color, Vec3}}, vars::{FIXFLT_VAL_1, FIXFLT_VAL_HALF1}};


const SKY_TOP_COLOR: Color = Color::new(
    FixFlt::lit(".459"),
    FixFlt::lit(".478"),
    FixFlt::lit(".749")
);
const SKY_BOTTOM_COLOR: Color = Color::new(
    FixFlt::lit(".918"),
    FixFlt::lit(".69"),
    FixFlt::lit(".82")
);

pub fn ray_color(r: &mut Ray) -> Color {
    let unit_dir = r.direction.unit_vec();
    let verticality = (unit_dir.y + FIXFLT_VAL_1) * FIXFLT_VAL_HALF1;
    SKY_BOTTOM_COLOR * (FIXFLT_VAL_1-verticality) + SKY_TOP_COLOR*verticality
}