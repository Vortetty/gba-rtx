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

mod vec3;
mod rand;
mod camera;
mod color;
mod ray;
mod utils;
mod trig_num;

use agb::{display, syscall, timer::{TimerController, Timer}, input::Button};
use agb_fixnum::{Num, num};
use color::Color;
use rand::{rand_u32};
use ray::Ray;
use trig_num::TrigFixedNum;
use vec3::Vec3;

fn hit_sphere(timer: &Timer, center: Vec3, radius: Num<i64, 20>, ray: &Ray) -> bool {
    let oc = ray.orig - center;
    let a = ray.dir.length_squared();
    let b = oc.dot_prod(ray.dir) * 2;
    let c = oc.length_squared() - radius*radius;
    let disc = b*b - a*c*4;
    return disc > Num::new(0);
}

fn ray_color(timer: &Timer, ray: &Ray) -> Color {
    if hit_sphere(timer, Vec3::newi(0, 0, -1), num!(0.5), ray) {
        return Color::new_01_range(Num::new(0), Num::new(1), Num::new(0));
    }
    if hit_sphere(timer, Vec3::newi(1, 0, -1), num!(0.5), ray) {
        return Color::new_01_range(Num::new(0), Num::new(0), Num::new(1));
    }
    if hit_sphere(timer, Vec3::newi(-1, 0, -1), num!(0.5), ray) {
        return Color::new_01_range(Num::new(1), Num::new(0), Num::new(0));
    }
    let unit_dir = ray.dir.unit_vector();
    let t = num!(0.5) * (unit_dir.y + 1);
    return Color::new_01_range(
        (num!(1.0)-t) * num!(1.0) + t*num!(0.5),
        (num!(1.0)-t) * num!(1.0) + t*num!(0.7),
        (num!(1.0)-t) * num!(1.0) + t*num!(1.0)
    );
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

    let aspect_ratio = Num::<i64, 20>::new(display::WIDTH as i64) / Num::<i64, 20>::new(display::HEIGHT as i64);
    let img_width = Num::<i64, 20>::new(display::WIDTH as i64);
    let img_height = Num::<i64, 20>::new(display::HEIGHT as i64);

    let viewport_height = num!(2.0);
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = num!(1.0);

    let orig = Vec3::newi(0, 0, 0);
    let horiz = Vec3::new(viewport_width, num!(0.0), num!(0.0));
    let vert = Vec3::new(num!(0.0), viewport_height, num!(0.0));
    let lower_left = orig - horiz/2 - vert/2 - Vec3::new(num!(0.0), num!(0.0), focal_length);

    for y in 0..display::HEIGHT as i64 {
        for x in 0..display::WIDTH as i64 {
            let u = Num::<i64, 20>::new(x) / (img_width-1);
            let v = Num::<i64, 20>::new(display::HEIGHT as i64-y) / (img_height-1);
            let ray = Ray{
                orig,
                dir: lower_left + u*horiz + v*vert - orig
            };
            let pc = ray_color(&t2, &ray);

            bitmap.draw_point(
                x as i32,
                y as i32,
                //oc
                (((pc.b * Num::new(31)).floor() as u16) << 10) + (((pc.g * Num::new(31)).floor() as u16) << 5) + (((pc.r * Num::new(31)).floor() as u16))
            );
        }
    }

    loop {
        syscall::halt();
    }
}
