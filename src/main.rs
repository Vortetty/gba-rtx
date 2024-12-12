/* This program is free software. It comes without any warranty, to
     * the extent permitted by applicable law. You can redistribute it
     * and/or modify it under the terms of the Do What The Fuck You Want
     * To Public License, Version 2, as published by Sam Hocevar. See
     * http://www.wtfpl.net/ for more details. */

// Games made using `agb` are no_std which means you don't have access to the standard
// rust library. This is because the game boy advance doesn't really have an operating
// system, so most of the content of the standard library doesn't apply.
//
// Provided you haven't disabled it, agb does provide an allocator, so it is possible
// to use both the `core` and the `alloc` built in crates.
#![no_std]
// `agb` defines its own `main` function, so you must declare your game's main function
// using the #[agb::entry] proc macro. Failing to do so will cause failure in linking
// which won't be a particularly clear error message.
#![no_main]
// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]
#![feature(allocator_api)]
#![allow(arithmetic_overflow)]

mod vec3;
mod rand;
mod camera;
mod color;
mod ray;
mod utils;
mod trig_num;
mod hittable;
mod hittables;
mod material;
mod mats;

pub extern crate alloc;

use core::f32::INFINITY;

use agb::{display::{self}, syscall, timer::{Timer}, mgba::{Mgba, DebugLevel}};

use alloc::{boxed::Box, vec::Vec};
use color::Color;
use hittable::{HitRecord, HittableList};
use hittables::sphere::Sphere;
use mats::{MatManager, lambertian::LambertianMat, dielectric::DielectricMat, metal::MetalMat};
use rand::{rand_double};
use ray::Ray;
use utils::{random_in_hemisphere};
use vec3::Vec3;
use trig_num::TrigNum;

#[inline(always)]
fn ray_color(timer: &Timer, ray: &Ray, world: &HittableList, mats: &MatManager, maxdepth: u32) -> Color {
    let mut rec: HitRecord = HitRecord::default();
    if maxdepth <= 0 {
        return Color::new_01_range(0.0, 0.0, 0.0);
    }

    if world.hit(timer, ray, 0.001, INFINITY, &mut rec) {
        //return Color::new_01_range(
        //    (rec.normal.x + (1.0)) * 0.5,
        //    (rec.normal.y + (1.0)) * 0.5,
        //    (rec.normal.z + (1.0)) * 0.5
        //);

        //let mut tgt = rec.point + random_in_hemisphere(&rec.normal, timer);
        //if tgt.near_zero() {
        //    tgt = rec.point;
        //}
        //let newray = Ray::new(rec.point, tgt-rec.point);
        //let lascol = ray_color(timer, &newray, world, maxdepth.clone()-1);
        //return Color {
        //    r: lascol.r * 0.5,
        //    g: lascol.g * 0.5,
        //    b: lascol.b * 0.5
        //};
        let mut scattered: Ray = Ray::new(Vec3::newi(0,0,0), Vec3::newi(0,0,0));
        let mut attenuation: Color = Color::new_01_range(0.0, 0.0, 0.0);
        if mats.get_mat(&rec.material).scatter(&ray, &mut rec, &mut attenuation, &mut scattered, timer) {
            let oc = ray_color(timer, &scattered, world, mats, maxdepth);
            return Color::new_01_range(
                oc.r * attenuation.r,
                oc.g * attenuation.g,
                oc.b * attenuation.b
            )
        }
        return Color::new_01_range(0.0, 0.0, 0.0);
    }
    let unit_dir = ray.dir.unit_vector();
    let t = (unit_dir.y + (1.0)) * 0.5;
    return Color::new_01_range(
        ((1.0)-t) * (1.0) + t*0.5,
        ((1.0)-t) * (1.0) + t*(0.7),
        ((1.0)-t) * (1.0) + t*(1.0)
    );
}

macro_rules! new_box {
    ($v: expr) => {
        Box::new($v)
    };
}

// The main function must take 1 arguments and never return. The agb::entry decorator
// ensures that everything is in order. `agb` will call this after setting up the stack
// and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let mut bitmap = gba.display.video.bitmap3();

    let mut t2 = gba.timers.timers().timer2;

    t2.set_divider(agb::timer::Divider::Divider64);
    t2.set_enabled(true);

    //let mut input = agb::input::ButtonController::new();
    //while !input.is_pressed(Button::START) {
    //    input.update();
    //}

    let aspect_ratio = (display::WIDTH as f32) / (display::HEIGHT as f32);
    let img_width = display::WIDTH as f32;
    let img_height = display::HEIGHT as f32;
    const ITERS: i32 = 5;
    const MAX_BOUNCES: u32 = 10;

    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let orig = Vec3::newi(0, 0, 0);
    let horiz = Vec3::new(viewport_width, 0.0, 0.0);
    let vert = Vec3::new(0.0, viewport_height, 0.0);
    let _lower_left = orig - (horiz * 0.5) - (vert * 0.5) - Vec3::new(0.0, 0.0, focal_length);
    let cam = camera::Camera::new(
        Vec3::newi(3, 3, 2),
        Vec3::newi(0, 0, -1),
        Vec3::newi(0, 1, 0),
        20.0,
        aspect_ratio,
        0.0,
        (Vec3::newi(3, 3, 2) - Vec3::newi(0, 0, -1)).length(),
        &t2
    );

    let mut world: HittableList = HittableList {
        objs: Vec::new()
    };
    let mut mats: MatManager = MatManager::new();

    world.add(
        new_box!(
            Sphere {
                center: Vec3::newi(0, 0, -1),
                radius: 0.5,
                material: mats.gen_mat(Box::new(
                    LambertianMat {
                        albedo: Color::new_01_range(0.1, 0.2, 0.5)
                    }
                ))
            }
        )
    );
    world.add(
        new_box!(
            Sphere {
                center: Vec3::newi(-1, 0, -1),
                radius: 0.5,
                material: mats.gen_mat(Box::new(
                    DielectricMat {
                        refract_index: 1.5
                    }
                ))
            }
        )
    );
    world.add(
        new_box!(
            Sphere {
                center: Vec3::newi(-1, 0, -1),
                radius: -0.45,
                material: mats.gen_mat(Box::new(
                    DielectricMat {
                        refract_index: 1.5
                    }
                ))
            }
        )
    );
    world.add(
        new_box!(
            Sphere {
                center: Vec3::newi(1, 0, -1),
                radius: 0.5,
                material: mats.gen_mat(Box::new(
                    MetalMat {
                        albedo: Color::new_01_range(0.8, 0.6, 0.2),
                        fuzz: 0.0
                    }
                ))
            }
        )
    );
    world.add(
        new_box!(
            Sphere {
                center: Vec3::new(
                    0.0,
                    -100.5,
                    -1.0
                ),
                radius: 100.0,
                material: mats.gen_mat(Box::new(
                    LambertianMat {
                        albedo: Color::new_01_range(0.8, 0.8, 0.0)
                    }
                ))
            }
        )
    );

    for y in 0..display::HEIGHT as i32 {
        for x in 0..display::WIDTH as i32 {
            let mut tc = Color::new(0.0, 0.0, 0.0);
            for _i in 0..ITERS {
                //let u = ((x as f32) + (rand_double(&t2))) / (img_width-(1.0));
                //let v = ((display::HEIGHT as f32-y as f32-1.0) + rand_double(&t2)) / (img_height-(1.0));
                let u = ((x as f32)) / (img_width-(1.0));
                let v = ((display::HEIGHT as i32-y) as f32 - 1.0) / (img_height-(1.0));
                let ray = cam.get_ray(u, v, &t2);
                let pc = ray_color(&t2, &ray, &world, &mats, MAX_BOUNCES);

                tc = Color::new_01_range(
                    tc.r + pc.r,
                    tc.g + pc.g,
                    tc.b + pc.b
                );
            }
            bitmap.draw_point(
                x as i32,
                y as i32,
                ((((tc.b/ITERS as f32).sqrt() * (31.0)) as u16) << 10) +
                    ((((tc.g/ITERS as f32).sqrt() * (31.0)) as u16) << 5) +
                    ((((tc.r/ITERS as f32).sqrt() * (31.0)) as u16))
            );
        }
    }

    loop {
        syscall::halt();
    }
}
