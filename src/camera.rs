use agb::{timer::Timer, display};
use agb_fixnum::{Num, num};
use fixed::types::I34F30;

use crate::{ray::Ray, utils::{deg_to_rad, random_in_unit_sphere, random_in_unit_disk}, vec3::Vec3, trig_num::TrigNum};

#[derive(Clone)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: I34F30
}

impl Camera {
    pub fn old_new() -> Camera{
        let aspect_ratio = I34F30::from_num(display::WIDTH as i32) / I34F30::from_num(display::HEIGHT as i32);
        let viewport_height = I34F30::from_num(2.0);
        let viewport_width = viewport_height * aspect_ratio;
        let focal_length = I34F30::from_num(1.0);

        let orig = Vec3::newi(0, 0, 0);
        let horiz = Vec3::new(viewport_width, I34F30::from_num(0), I34F30::from_num(0));
        let vert = Vec3::new(I34F30::from_num(0), viewport_width, I34F30::from_num(0));

        return Camera {
            origin: orig,
            horizontal: horiz,
            vertical: vert,
            lower_left_corner: orig - (horiz >> 1) - (vert >> 1) - Vec3::new(I34F30::from_num(0.0), I34F30::from_num(0.0), focal_length),
            u: Vec3::newi(0, 0, 0),
            v: Vec3::newi(0, 0, 0),
            w: Vec3::newi(0, 0, 0),
            lens_radius: I34F30::from_num(0)
        };
    }

    pub fn new(
        look_from: Vec3,
        look_to: Vec3,
        view_up: Vec3,
        vert_fov: I34F30,
        aspect_ratio: I34F30,
        aperture: I34F30,
        focus_dist: I34F30,
        rng: &Timer
    ) -> Camera {
        let theta = deg_to_rad(vert_fov);
        let h = (theta / I34F30::from_num(2.0)).tan();
        //let aspect_ratio: I34F30 = 16.0 / 9.0;
        let viewport_height: I34F30 = I34F30::from_num(2.0) * h;
        let viewport_width: I34F30 = aspect_ratio * viewport_height;

        let w = (look_from - look_to).unit_vector();
        let u = view_up.cross_prod(w).unit_vector();
        let v = w.cross_prod(u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - (horizontal>>1) - (vertical>>1) - focus_dist * w;

        return Camera {
            origin: origin,
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
            w,
            u,
            v,
            lens_radius: aperture/I34F30::from_num(2.0)
        };
    }

    pub fn get_ray(&self, s: I34F30, t: I34F30, rng: &Timer) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(rng);
        let offset = self.u * rd.x + self.v * rd.y;

        return Ray {
            orig: self.origin + offset,
            //orig: self.origin,
            dir: self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            //dir: self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin
        };
    }

    pub fn default(rng: &Timer) -> Self {
        let aspect_ratio = I34F30::from_num(16.0) / I34F30::from_num(9.0);
        let look_from = Vec3::newi(0, 0, 0);
        let look_to = Vec3::newi(0, 0, -1);
        let view_up = Vec3::newi(0, 1, 0);
        let focus_dist = (look_from - look_to).length();
        let aperture = I34F30::from_num(0.0);
        let vert_fov = I34F30::from_num(35.0);

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
