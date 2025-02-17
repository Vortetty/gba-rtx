#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]
#![allow(incomplete_features)]
#![allow(internal_features)]
#![allow(unused)] // Lol we should disable this some day
#![feature(generic_const_exprs)]
#![feature(core_intrinsics)]
#![feature(variant_count)]
#![feature(f16)]

mod vars;
mod get_render_config;
mod resources;
mod math;
mod tracer;

#[macro_use]
extern crate alloc;

use core::time::Duration;

use get_render_config::{RenderConfig, Scenes};
use resources::pixelara::PIXELARA;
use vars::GBA_SCREEN_1_OVER_Y;
use agb::{fixnum::FixedNum, sound::mixer::Frequency, timer::Timer};
use math::types::FixFlt;
use agb::{sound::mixer::SoundChannel, timer::Divider};
use tracer::render;
use vars::GBA_SCREEN_X;
use core::ptr::write_volatile;

const MEMORY_SPEED_SHI: *mut u16 = 0x04000204 as *mut u16;

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    // Basics needed for gui
    unsafe {
        write_volatile(MEMORY_SPEED_SHI, 0x4317);
    }

    use math::vec3::Vec3;
    use vars::{GBA_SCREEN_X_I32, GBA_SCREEN_Y_I32};
    let mut bitmap = gba.display.video.bitmap3();
    let mut input = agb::input::ButtonController::new();
    let mut timers = gba.timers.timers();
    let mut timer2: Timer = timers.timer2;
    let mut timer3: Timer = timers.timer3;

    // Get configuration for renderer
    //let conf = get_render_config::get_render_config(&mut input, &mut bitmap);
    let conf = RenderConfig {
        scene: Scenes::SPHERES,
        iters_per_pixel: 8,
        max_depth: 16,
        hd_mode: false
    };
    bitmap.clear(0);

    timer2.set_divider(Divider::Divider1024);
    timer3.set_divider(Divider::Divider1);
    timer3.set_cascade(true);
    timer3.set_enabled(true);
    timer2.set_enabled(true);

    // Disable to re-enable music, we can take care of music later
    //mixer.channel(&channel_id).unwrap().stop();


    let focal_length = FixFlt::one();


    let viewport_height = FixFlt::from_i32(2);

    let viewport_width = viewport_height * (GBA_SCREEN_X * GBA_SCREEN_1_OVER_Y);

    render(&mut bitmap, viewport_height, viewport_width, focal_length, conf);

    timer2.set_enabled(false);
    timer3.set_enabled(false);

    let total_cycles = (timer3.value() as u32) << 16 | timer2.value() as u32;
    let time_per_1024_cycles = Duration::from_nanos(61035); // 61035.15625ns per 1024 clock cycles
    let total_time = total_cycles * time_per_1024_cycles;
    //let total_time = Duration::from_nanos((61035.15625 * total_cycles as f64) as u64);

    PIXELARA.print_str_rel(format!("{:.03}s", total_time.as_secs_f64()), &mut bitmap, 0, 0);

    //PIXELARA.print_str_rel(format!("{:}", FixFlt::from_i32(65536/16).recip().as_f32()), &mut bitmap, 0, 0);
    //PIXELARA.print_str_rel(format!("{:}", 1.0/(65536.0/16.0)), &mut bitmap, 0, 1);
    loop {
    }
}