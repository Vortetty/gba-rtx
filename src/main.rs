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
#![allow(incomplete_features)]
#![allow(internal_features)]
#![feature(generic_const_exprs)]
#![feature(core_intrinsics)]
#![feature(variant_count)]

mod vars;
mod get_render_config;
mod resources;
mod math;
mod tracer;

#[macro_use]
extern crate alloc;

use get_render_config::{RenderConfig, Scenes};
use resources::{music::LOFI_LOOP, pixelara::PIXELARA};
use vars::{GBA_SCREEN_1_OVER_X, GBA_SCREEN_1_OVER_Y, GBA_SCREEN_X_I32, GBA_SCREEN_Y_I32};
use agb::sound::mixer::{Frequency, SoundChannel};
use math::types::FixFlt;

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    // Basics needed for gui

    use tracer::render;
    use vars::GBA_SCREEN_X;
    let mut bitmap = gba.display.video.bitmap3();
    let mut input = agb::input::ButtonController::new();

    // Music setup
    let mut mixer = gba.mixer.mixer(Frequency::Hz10512);
    let mut channel = SoundChannel::new(LOFI_LOOP);
    channel.should_loop();
    mixer.enable();
    let channel_id = mixer.play_sound(channel).unwrap();

    // Get configuration for renderer
    //let conf = get_render_config::get_render_config(&mut input, &mut bitmap, &mut mixer);
    let conf = RenderConfig {
        scene: Scenes::SPHERES,
        iters_per_pixel: 4,
        max_depth: 8
    };
    bitmap.clear(0);

    // Color test screen
    //for y in 0..GBA_SCREEN_Y_I32 {
    //    let y_fix: FixFlt = y*GBA_SCREEN_1_OVER_Y;
    //    let y_fix_31_round = (y_fix * 31).round().to_num::<u16>();
    //    for x in  0..GBA_SCREEN_X_I32 {
    //        let x_fix: FixFlt = x*GBA_SCREEN_1_OVER_X;
    //        let x_fix_31_round = (x_fix * 31).round().to_num::<u16>();

    //        let mut px = 0;

    //        px |= (31-x_fix_31_round) << 10;
    //        px |= x_fix_31_round << 5;
    //        px |= y_fix_31_round;

    //        bitmap.draw_point(x as i32, y as i32, px);
    //    }
    //    mixer.frame();
    //}

    // Disable to re-enable music, we can take care of music later
    mixer.channel(&channel_id).unwrap().stop();

    let focal_length = FixFlt::lit("1.0");

    let viewport_height = FixFlt::lit("2.0");
    let viewport_width = viewport_height * (GBA_SCREEN_X * GBA_SCREEN_1_OVER_Y);

    render(&mut bitmap, viewport_height, viewport_width, focal_length);

    loop {
        mixer.frame(); // Play music forever
    }
}
