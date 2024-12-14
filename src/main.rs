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
#![feature(generic_const_exprs)]

mod utils;
mod get_render_config;

use core::arch::asm;
use fixed::types::I14F18;
use utils::{GBA_SCREEN_1_OVER_X, GBA_SCREEN_1_OVER_Y, GBA_SCREEN_X_I32, GBA_SCREEN_Y_I32};

// The main function must take 1 arguments and never return. The agb::entry decorator
// ensures that everything is in order. `agb` will call this after setting up the stack
// and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let mut bitmap = gba.display.video.bitmap3();

    for y in 0..GBA_SCREEN_Y_I32 {
        let y_fix: I14F18 = y*GBA_SCREEN_1_OVER_Y;
        for x in  0..GBA_SCREEN_X_I32 {
            let x_fix: I14F18 = x*GBA_SCREEN_1_OVER_X;

            let mut px = 0;

            px += ((I14F18::lit("1") - x_fix) * 31).round().to_num::<u16>() << 10;
            px += (x_fix * 31).round().to_num::<u16>() << 5;
            px += (y_fix * 31).round().to_num::<u16>();

            bitmap.draw_point(x as i32, y as i32, px);
        }
    }

    loop {
        unsafe {
            asm!(
                "nop"
            )
        }
    }
}
