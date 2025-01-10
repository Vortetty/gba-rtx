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

use core::time::Duration;
use micromath::F32Ext;

use get_render_config::{RenderConfig, Scenes};
use resources::{music::LOFI_LOOP, pixelara::PIXELARA};
use vars::{GBA_SCREEN_1_OVER_X, GBA_SCREEN_1_OVER_Y, GBA_SCREEN_X_I32, GBA_SCREEN_Y_I32};
use agb::{println, sound::mixer::{Frequency, SoundChannel}, timer::{self, Timer}};
use math::types::FixFlt;

#[link_section = ".iwram"]
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    // Basics needed for gui

    use agb::timer::Divider;
    use tracer::render;
    use vars::GBA_SCREEN_X;
    let mut bitmap = gba.display.video.bitmap3();
    let mut input = agb::input::ButtonController::new();
    let mut timers = gba.timers.timers();
    let mut timer2: Timer = timers.timer2;
    let mut timer3: Timer = timers.timer3;

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

    timer2.set_divider(Divider::Divider1024);
    timer3.set_divider(Divider::Divider1);
    timer3.set_cascade(true);
    timer3.set_enabled(true);
    timer2.set_enabled(true);

    // Disable to re-enable music, we can take care of music later
    mixer.channel(&channel_id).unwrap().stop();

    let focal_length = FixFlt::one();

    let viewport_height = FixFlt::from_i32(2);
    let viewport_width = viewport_height * (GBA_SCREEN_X * GBA_SCREEN_1_OVER_Y);

    render(&mut bitmap, viewport_height, viewport_width, focal_length, &mut mixer);

    timer2.set_enabled(false);
    timer3.set_enabled(false);

    let total_cycles = (timer3.value() as u32) << 16 | timer2.value() as u32;
    let time_per_1024_cycles = Duration::from_nanos(61035); // 61035.15625ns per 1024 clock cycles
    let total_time = total_cycles * time_per_1024_cycles;

    PIXELARA.print_str(format!("{:.03}s", total_time.as_millis() as f64/1000.0), &mut bitmap, 0, 0);

    // PIXELARA.print_str_rel(format!("{:}", FixFlt::from_f32(128.0).recip().as_f32()), &mut bitmap, 0, 0);
    // PIXELARA.print_str_rel(format!("{:}", 1.0/128.0), &mut bitmap, 0, 1);

    loop {
        mixer.frame(); // Play music forever
    }
}
