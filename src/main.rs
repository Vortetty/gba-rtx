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

use agb::{display, syscall, timer::TimerController, input::Button};
use agb_fixnum::{Num, num};
use rand::{rand_u32, seed_rng_from_timer};

// The main function must take 1 arguments and never return. The agb::entry decorator
// ensures that everything is in order. `agb` will call this after setting up the stack
// and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let mut bitmap = gba.display.video.bitmap3();

    //for x in 0..display::WIDTH {
    //    let y: Num<i32, 16> = Num::sin(Num::new(x)/150)/2+num!(0.5);//syscall::sqrt(x << 6);
    //    let y: Num<i32, 16> = (Num::new(display::HEIGHT) * y).clamp(num!(0.0), Num::new(display::HEIGHT) - 1);
    //    bitmap.draw_point(x, y.floor(), 0x001F);
    //}

    //for x in 0..display::WIDTH {
    //    for y in 0..display::HEIGHT {
    //        let mut color: u16 = 0;
    //        color += (Num::<i32, 8>::new(31)/display::WIDTH * (display::WIDTH-x)).floor() as u16;
    //        color += ((Num::<i32, 8>::new(31)/display::WIDTH * x).floor() as u16) << 10;
    //        color += ((Num::<i32, 8>::new(31)/display::HEIGHT * y).floor() as u16) << 5;

    //        bitmap.draw_point(x, y, color);
    //    }
    //}
    let mut t2 = gba.timers.timers().timer2;

    t2.set_divider(agb::timer::Divider::Divider64);
    t2.set_enabled(true);

    loop {
        let mut input = agb::input::ButtonController::new();
        while !input.is_pressed(Button::START) {
            input.update();
        }

        seed_rng_from_timer(&t2);

        for x in 0..display::WIDTH {
            for y in 0..display::HEIGHT {
                bitmap.draw_point(x, y, rand_u32() as u16);
            }
        }
    }

    loop {
        syscall::halt();
    }
}
