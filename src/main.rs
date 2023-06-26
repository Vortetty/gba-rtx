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
use rand::{rand_u32};
use ray::Ray;
use fixed::types::I14F18;
use vec3::Vec3;

fn ray_color(timer: &Timer, ray: &Ray, world: &HittableList) -> Color {
    let mut rec: HitRecord = HitRecord::default();
    if world.hit(timer, ray, I14F18::from_num(0), I14F18::MAX, &mut rec) {
        return Color::new_01_range(
            (rec.normal.x + I14F18::from_num(1)) >> 1,
            (rec.normal.y + I14F18::from_num(1)) >> 1,
            (rec.normal.z + I14F18::from_num(1)) >> 1
        );
    }
    let unit_dir = ray.dir.unit_vector();
    let t = (unit_dir.y + I14F18::from_num(1)) >> 1;
    return Color::new_01_range(
        (I14F18::from_num(1.0)-t) * I14F18::from_num(1.0) + t*I14F18::from_num(0.5),
        (I14F18::from_num(1.0)-t) * I14F18::from_num(1.0) + t*I14F18::from_num(0.7),
        (I14F18::from_num(1.0)-t) * I14F18::from_num(1.0) + t*I14F18::from_num(1.0)
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

    let aspect_ratio = I14F18::from_num(display::WIDTH as i32) / I14F18::from_num(display::HEIGHT as i32);
    let img_width = I14F18::from_num(display::WIDTH as i32);
    let img_height = I14F18::from_num(display::HEIGHT as i32);

    let viewport_height = I14F18::from_num(2.0);
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = I14F18::from_num(1.0);

    let orig = Vec3::newi(0, 0, 0);
    let horiz = Vec3::new(viewport_width, I14F18::from_num(0.0), I14F18::from_num(0.0));
    let vert = Vec3::new(I14F18::from_num(0.0), viewport_height, I14F18::from_num(0.0));
    let lower_left = orig - (horiz >> 1) - (vert >> 1) - Vec3::new(I14F18::from_num(0.0), I14F18::from_num(0.0), focal_length);

    let mut world: HittableList = HittableList {
        objs: Vec::new()
    };

    world.add(
        new_box!(
            Sphere {
                center: Vec3::newi(0, 0, -1),
                radius: I14F18::from_num(0.5)
            }
        )
    );

    for y in 0..display::HEIGHT as i32 {
        for x in 0..display::WIDTH as i32 {
            let u = I14F18::from_num(x) / (img_width-I14F18::from_num(1));
            let v = I14F18::from_num(display::HEIGHT as i32-y) / (img_height-I14F18::from_num(1));
            let ray = Ray{
                orig,
                dir: lower_left + u*horiz + v*vert - orig
            };
            let pc = ray_color(&t2, &ray, &world);

            bitmap.draw_point(
                x as i32,
                y as i32,
                //oc
                (((pc.b * I14F18::from_num(31)).floor().to_num::<u16>() as u16) << 10) + (((pc.g * I14F18::from_num(31)).floor().to_num::<u16>() as u16) << 5) + (((pc.r * I14F18::from_num(31)).floor().to_num::<u16>() as u16))
            );
        }
    }

    loop {
        syscall::halt();
    }
}
