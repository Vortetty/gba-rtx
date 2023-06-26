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
#![feature(generic_const_exprs)]

mod vec3;
mod rand;
mod camera;
mod color;
mod ray;
mod utils;
mod trig_num;
mod hittable;
mod hittables;

pub extern crate alloc;

use agb::{display, syscall, timer::{TimerController, Timer}, input::Button, ExternalAllocator};
use agb_fixnum::{Num, num};
use alloc::{boxed::Box, vec::Vec};
use color::Color;
use hittable::{HitRecord, HittableList, Hittable};
use hittables::sphere::Sphere;
use rand::{rand_u32, rand_double};
use ray::Ray;
use fixed::types::I20F12;
use vec3::Vec3;

fn ray_color(timer: &Timer, ray: &Ray, world: &HittableList) -> Color {
    let mut rec: HitRecord = HitRecord::default();
    if world.hit(timer, ray, I20F12::from_num(0), I20F12::MAX, &mut rec) {
        return Color::new_01_range(
            (rec.normal.x + I20F12::from_num(1)) >> 1,
            (rec.normal.y + I20F12::from_num(1)) >> 1,
            (rec.normal.z + I20F12::from_num(1)) >> 1
        );
    }
    let unit_dir = ray.dir.unit_vector();
    let t = (unit_dir.y + I20F12::from_num(1)) >> 1;
    return Color::new_01_range(
        (I20F12::from_num(1.0)-t) * I20F12::from_num(1.0) + t*I20F12::from_num(0.5),
        (I20F12::from_num(1.0)-t) * I20F12::from_num(1.0) + t*I20F12::from_num(0.7),
        (I20F12::from_num(1.0)-t) * I20F12::from_num(1.0) + t*I20F12::from_num(1.0)
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

    t2.set_divider(agb::timer::Divider::Divider1);
    t2.set_enabled(true);

    //let mut input = agb::input::ButtonController::new();
    //while !input.is_pressed(Button::START) {
    //    input.update();
    //}

    let aspect_ratio = I20F12::from_num(display::WIDTH as i32) / I20F12::from_num(display::HEIGHT as i32);
    let img_width = I20F12::from_num(display::WIDTH as i32);
    let img_height = I20F12::from_num(display::HEIGHT as i32);
    const ITERS: i32 = 5;

    let viewport_height = I20F12::from_num(2.0);
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = I20F12::from_num(1.0);

    let orig = Vec3::newi(0, 0, 0);
    let horiz = Vec3::new(viewport_width, I20F12::from_num(0.0), I20F12::from_num(0.0));
    let vert = Vec3::new(I20F12::from_num(0.0), viewport_height, I20F12::from_num(0.0));
    let lower_left = orig - (horiz >> 1) - (vert >> 1) - Vec3::new(I20F12::from_num(0.0), I20F12::from_num(0.0), focal_length);
    let mut cam = camera::Camera::new(
        Vec3::newi(3, 3, 2),
        Vec3::newi(0, 0, -1),
        Vec3::newi(0, 1, 0),
        I20F12::from_num(20),
        aspect_ratio,
        I20F12::from_num(1),
        (Vec3::newi(3, 3, 2) - Vec3::newi(0, 0, -1)).length(),
        &t2
    );

    let mut world: HittableList = HittableList {
        objs: Vec::new()
    };

    world.add(
        new_box!(
            Sphere {
                center: Vec3::newi(0, 0, -1),
                radius: I20F12::from_num(0.5)
            }
        )
    );
    world.add(
        new_box!(
            Sphere {
                center: Vec3::newi(-1, 0, -1),
                radius: I20F12::from_num(0.5)
            }
        )
    );
    world.add(
        new_box!(
            Sphere {
                center: Vec3::newi(1, 0, -1),
                radius: I20F12::from_num(0.5)
            }
        )
    );
    world.add(
        new_box!(
            Sphere {
                center: Vec3::new(
                    I20F12::from_num(0),
                    I20F12::from_num(-100.5),
                    I20F12::from_num(-1)
                ),
                radius: I20F12::from_num(100)
            }
        )
    );

    for y in 0..display::HEIGHT as i32 {
        for x in 0..display::WIDTH as i32 {
            let mut tc = Color::new(I20F12::from_num(0), I20F12::from_num(0), I20F12::from_num(0));
            for _i in 0..ITERS {
                let u = (I20F12::from_num(x) + (rand_double(&t2) >> 1)) / (img_width-I20F12::from_num(1));
                let v = (I20F12::from_num(display::HEIGHT as i32-y-1) + (rand_double(&t2) >> 1)) / (img_height-I20F12::from_num(1));
                //let u = (I20F12::from_num(x)) / (img_width-I20F12::from_num(1));
                //let v = (I20F12::from_num(display::HEIGHT as i32-y)) / (img_height-I20F12::from_num(1));
                let ray = cam.get_ray(u, v, &t2);
                let pc = ray_color(&t2, &ray, &world);

                tc = Color::new_01_range(
                    tc.r + pc.r,
                    tc.g + pc.g,
                    tc.b + pc.b
                );
            }
            bitmap.draw_point(
                x as i32,
                y as i32,
                ((((tc.b/ITERS) * I20F12::from_num(31)).floor().to_num::<u16>() as u16) << 10) +
                    ((((tc.g/ITERS) * I20F12::from_num(31)).floor().to_num::<u16>() as u16) << 5) +
                    ((((tc.r/ITERS) * I20F12::from_num(31)).floor().to_num::<u16>() as u16))
            );
        }
    }

    loop {
        syscall::halt();
    }
}
