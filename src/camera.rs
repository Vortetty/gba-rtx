use agb::timer::Timer;
use agb_fixnum::{Num, num};

use crate::{ray::Ray, utils::{deg_to_rad, random_in_unit_sphere, random_in_unit_disk}, vec3::Vec3, trig_num::TrigFixedNum};

#[derive(Clone)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: Num<i64, 20>
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_to: Vec3,
        view_up: Vec3,
        vert_fov: Num<i64, 20>,
        aspect_ratio: Num<i64, 20>,
        aperture: Num<i64, 20>,
        focus_dist: Num<i64, 20>,
        rng: &Timer
    ) -> Camera {
        let theta = deg_to_rad(vert_fov);
        let h = (theta / num!(2.0)).tan();
        //let aspect_ratio: Num<i64, 20> = 16.0 / 9.0;
        let viewport_height: Num<i64, 20> = num!(2.0) * h;
        let viewport_width: Num<i64, 20> = aspect_ratio * viewport_height;

        let w = (look_from - look_to).unit_vector();
        let u = view_up.cross_prod(w).unit_vector();
        let v = w.cross_prod(u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2 - vertical / 2 - focus_dist * w;

        return Camera {
            origin: origin,
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
            w,
            u,
            v,
            lens_radius: aperture/num!(2.0)
        };
    }

    pub fn get_ray(&self, s: Num<i64, 20>, t: Num<i64, 20>, rng: &Timer) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(rng);
        let offset = self.u * rd.x + self.v * rd.y;

        return Ray {
            orig: self.origin + offset,
            dir: self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        };
    }

    pub fn default(rng: &Timer) -> Self {
        let aspect_ratio = num!(16.0) / num!(9.0);
        let look_from = Vec3::newi(0, 0, 0);
        let look_to = Vec3::newi(0, 0, -1);
        let view_up = Vec3::newi(0, 1, 0);
        let focus_dist = (look_from - look_to).length();
        let aperture = num!(0.0);
        let vert_fov = num!(35.0);

        return Self::new(
            look_from,
            look_to,
            view_up,
            vert_fov,
            aspect_ratio,
            aperture,
            focus_dist,
            rng
        );
    }
}
