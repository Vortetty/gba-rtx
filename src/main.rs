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
use agb::{sound::mixer::Frequency, timer::Timer};
use math::types::FixFlt;
use agb::{sound::mixer::SoundChannel, timer::Divider};
use resources::music::LOFI_LOOP;
use tracer::render;
use vars::GBA_SCREEN_X;

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    // Basics needed for gui

    use math::vec3::Vec3;
    use vars::{GBA_SCREEN_X_I32, GBA_SCREEN_Y_I32};
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
        iters_per_pixel: 8,
        max_depth: 8
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

    render(&mut bitmap, viewport_height, viewport_width, focal_length, &mut mixer, conf);

    #[link_section = ".ewram"]
    static mut FRAMEBUFFER_1: [[u16; GBA_SCREEN_X_I32 as usize]; GBA_SCREEN_Y_I32 as usize] =  [[0u16; GBA_SCREEN_X_I32 as usize]; GBA_SCREEN_Y_I32 as usize];
    for y in 0..GBA_SCREEN_Y_I32 as usize {
        for x in 0..GBA_SCREEN_X_I32 as usize {
            unsafe {
                FRAMEBUFFER_1[y][x] = bitmap.read_point(x as i32, y as i32);
            }
        }
    }
    for y in 0..GBA_SCREEN_Y_I32 {
        for x in 0..GBA_SCREEN_X_I32 {
            unsafe {
                let color = Vec3::from_gba_color(FRAMEBUFFER_1[y as usize][x as usize]);
                let up = Vec3::from_gba_color(FRAMEBUFFER_1[(y+1).clamp(0, const{GBA_SCREEN_Y_I32-1}) as usize][x as usize]);
                let down = Vec3::from_gba_color(FRAMEBUFFER_1[(y-1).clamp(0, const{GBA_SCREEN_Y_I32-1}) as usize][x as usize]);
                let right = Vec3::from_gba_color(FRAMEBUFFER_1[y as usize][(x+1).clamp(0, const{GBA_SCREEN_X_I32-1}) as usize]);
                let left = Vec3::from_gba_color(FRAMEBUFFER_1[y as usize][(x-1).clamp(0, const{GBA_SCREEN_X_I32-1}) as usize]);

                if (color.luma() - up.luma()).abs()     > FixFlt::from_f32(0.025) &&
                    (color.luma() - down.luma()).abs()  > FixFlt::from_f32(0.025) &&
                    (color.luma() - right.luma()).abs() > FixFlt::from_f32(0.025) &&
                    (color.luma() - left.luma()).abs()  > FixFlt::from_f32(0.025) {
                    bitmap.draw_point(x, y, ((up + down + right + left) * FixFlt::from(0.25)).to_gba_color());
                }
            }
        }
    }
    for y in 0..GBA_SCREEN_Y_I32 as usize {
        for x in 0..GBA_SCREEN_X_I32 as usize {
            unsafe {
                FRAMEBUFFER_1[y][x] = bitmap.read_point(x as i32, y as i32);
            }
        }
    }
    for y in 0..GBA_SCREEN_Y_I32 {
        for x in 0..GBA_SCREEN_X_I32 {
            unsafe {
                let mut color = Vec3::from_gba_color(FRAMEBUFFER_1[y as usize][x as usize]);
                let up = Vec3::from_gba_color(FRAMEBUFFER_1[(y+1).clamp(0, const{GBA_SCREEN_Y_I32-1}) as usize][x as usize]);
                let down = Vec3::from_gba_color(FRAMEBUFFER_1[(y-1).clamp(0, const{GBA_SCREEN_Y_I32-1}) as usize][x as usize]);
                let right = Vec3::from_gba_color(FRAMEBUFFER_1[y as usize][(x+1).clamp(0, const{GBA_SCREEN_X_I32-1}) as usize]);
                let left = Vec3::from_gba_color(FRAMEBUFFER_1[y as usize][(x-1).clamp(0, const{GBA_SCREEN_X_I32-1}) as usize]);

                let mut avgcnt = 0;
                let mut tmpcolor = color-color;

                if (color.luma() - up.luma()).abs() < FixFlt::from_f32(0.1) {
                    tmpcolor = tmpcolor + up;
                    avgcnt += 1;
                }
                if (color.luma() - down.luma()).abs() < FixFlt::from_f32(0.1) {
                    tmpcolor = tmpcolor + down;
                    avgcnt += 1;
                }
                if (color.luma() - right.luma()).abs() < FixFlt::from_f32(0.1) {
                    tmpcolor = tmpcolor + right;
                    avgcnt += 1;
                }
                if (color.luma() - left.luma()).abs() < FixFlt::from_f32(0.1) {
                    tmpcolor = tmpcolor + left;
                    avgcnt += 1;
                }

                if (avgcnt > 0) {
                    bitmap.draw_point(x, y, ((tmpcolor) / FixFlt::from_i32(avgcnt)).to_gba_color())
                }
            }
        }
    }
    //for y in 0..GBA_SCREEN_Y_I32 as usize {
    //    for x in 0..GBA_SCREEN_X_I32 as usize {
    //        unsafe {
    //            FRAMEBUFFER_1[y][x] = bitmap.read_point(x as i32, y as i32);
    //        }
    //    }
    //}
    //for y in 0..GBA_SCREEN_Y_I32 {
    //    for x in 0..GBA_SCREEN_X_I32 {
    //        unsafe {
    //            let mut color = Vec3::from_gba_color(FRAMEBUFFER_1[y as usize][x as usize]);
    //            let up = Vec3::from_gba_color(FRAMEBUFFER_1[(y+1).clamp(0, const{GBA_SCREEN_Y_I32-1}) as usize][x as usize]);
    //            let down = Vec3::from_gba_color(FRAMEBUFFER_1[(y-1).clamp(0, const{GBA_SCREEN_Y_I32-1}) as usize][x as usize]);
    //            let right = Vec3::from_gba_color(FRAMEBUFFER_1[y as usize][(x+1).clamp(0, const{GBA_SCREEN_X_I32-1}) as usize]);
    //            let left = Vec3::from_gba_color(FRAMEBUFFER_1[y as usize][(x-1).clamp(0, const{GBA_SCREEN_X_I32-1}) as usize]);

    //            let mut avgcnt = 1;
    //            let mut tmpcolor = color;

    //            if (color.luma() - up.luma()).abs() < FixFlt::from_f32(0.1) {
    //                tmpcolor = tmpcolor + up;
    //                avgcnt += 1;
    //            }
    //            if (color.luma() - down.luma()).abs() < FixFlt::from_f32(0.1) {
    //                tmpcolor = tmpcolor + down;
    //                avgcnt += 1;
    //            }
    //            if (color.luma() - right.luma()).abs() < FixFlt::from_f32(0.1) {
    //                tmpcolor = tmpcolor + right;
    //                avgcnt += 1;
    //            }
    //            if (color.luma() - left.luma()).abs() < FixFlt::from_f32(0.1) {
    //                tmpcolor = tmpcolor + left;
    //                avgcnt += 1;
    //            }

    //            if (avgcnt > 0) {
    //                bitmap.draw_point(x, y, ((tmpcolor) / FixFlt::from_i32(avgcnt)).to_gba_color())
    //            }
    //        }
    //    }
    //}

    timer2.set_enabled(false);
    timer3.set_enabled(false);

    let total_cycles = (timer3.value() as u32) << 16 | timer2.value() as u32;
    let time_per_1024_cycles = Duration::from_nanos(61035); // 61035.15625ns per 1024 clock cycles
    let total_time = total_cycles * time_per_1024_cycles;

    PIXELARA.print_str(format!("{:.03}s", total_time.as_millis() as f64/1000.0), &mut bitmap, 0, 0);

    //PIXELARA.print_str_rel(format!("{:}", FixFlt::from_i32(65536/16).recip().as_f32()), &mut bitmap, 0, 0);
    //PIXELARA.print_str_rel(format!("{:}", 1.0/(65536.0/16.0)), &mut bitmap, 0, 1);

    loop {
        mixer.frame(); // Play music forever
    }
}