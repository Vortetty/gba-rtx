use core::ops::AddAssign;

use agb::{display::bitmap3::Bitmap3, sound::mixer::Mixer};

use crate::{
    math::{types::FixFlt, vec3::Vec3},
    vars::{GBA_SCREEN_X_I32, GBA_SCREEN_Y_I32},
};

#[repr(C, align(4))]
pub struct AlignedBuffer([u8; 240 * 160 * 3]);

#[link_section = ".ewram"] // Can hold a full rgb888 framebuffer or 555 framebuffer, perfect for both the low and high res denoisers
pub static mut FRAMEBUFFER_1_STATIC: AlignedBuffer = AlignedBuffer([0; 240 * 160 * 3]);

// Safe access to the u16 view (160 * 2 bytes per row)
#[allow(static_mut_refs)]
fn as_u16_view_mut() -> &'static mut [[u16; 240]; 160] {
    unsafe { &mut *(FRAMEBUFFER_1_STATIC.0.as_mut_ptr() as *mut [[u16; 240]; 160]) }
}

// Safe access to the u8 view (160 * 3 bytes per row)
#[allow(static_mut_refs)]
pub fn as_rgb_view_mut() -> &'static mut [[[u8; 3]; 240]; 160] {
    unsafe { &mut *(FRAMEBUFFER_1_STATIC.0.as_mut_ptr() as *mut [[[u8; 3]; 240]; 160]) }
}

pub fn denoise(bitmap: &mut Bitmap3) {
    let framebuffer_1 = as_u16_view_mut();
    //
    // Basic acne removal, checks the 4 immediate neighbors and if 3
    //    are more than 5% different (by luminance) from the current pixel, replaces the current pixel with the average of neighbors
    //
    for y in 0..GBA_SCREEN_Y_I32 as usize {
        for x in 0..GBA_SCREEN_X_I32 as usize {
            unsafe {
                framebuffer_1[y][x] = bitmap.read_point(x as i32, y as i32);
            }
        }
    }
    for y in 0..GBA_SCREEN_Y_I32 {
        for x in 0..GBA_SCREEN_X_I32 {
            unsafe {
                let color = Vec3::from_gba_color(framebuffer_1[y as usize][x as usize]);
                let up = Vec3::from_gba_color(
                    framebuffer_1[(y + 1).clamp(0, const { GBA_SCREEN_Y_I32 - 1 }) as usize]
                        [x as usize],
                );
                let down = Vec3::from_gba_color(
                    framebuffer_1[(y - 1).clamp(0, const { GBA_SCREEN_Y_I32 - 1 }) as usize]
                        [x as usize],
                );
                let right = Vec3::from_gba_color(
                    framebuffer_1[y as usize]
                        [(x + 1).clamp(0, const { GBA_SCREEN_X_I32 - 1 }) as usize],
                );
                let left = Vec3::from_gba_color(
                    framebuffer_1[y as usize]
                        [(x - 1).clamp(0, const { GBA_SCREEN_X_I32 - 1 }) as usize],
                );

                let m = ((color.luma() - up.luma()).abs() > FixFlt::from_f32(0.05)) as u8
                    + ((color.luma() - down.luma()).abs() > FixFlt::from_f32(0.05)) as u8
                    + ((color.luma() - right.luma()).abs() > FixFlt::from_f32(0.05)) as u8
                    + ((color.luma() - left.luma()).abs() > FixFlt::from_f32(0.05)) as u8;

                if m >= 3 {
                    bitmap.draw_point(
                        x,
                        y,
                        ((up + down + right + left) * FixFlt::from(0.25)).to_gba_color(),
                    );
                }
            }
        }
    }

    //
    // Basic edge-aware denoise.
    // replaces center pixel with the average of the neighbor pixels that are less than 10% different from the current.
    // makes it look dithered
    //
    for y in 0..GBA_SCREEN_Y_I32 as usize {
        for x in 0..GBA_SCREEN_X_I32 as usize {
            unsafe {
                framebuffer_1[y][x] = bitmap.read_point(x as i32, y as i32);
            }
        }
    }
    for y in 0..GBA_SCREEN_Y_I32 {
        for x in 0..GBA_SCREEN_X_I32 {
            unsafe {
                let mut color = Vec3::from_gba_color(framebuffer_1[y as usize][x as usize]);
                let up = Vec3::from_gba_color(
                    framebuffer_1[(y + 1).clamp(0, const { GBA_SCREEN_Y_I32 - 1 }) as usize]
                        [x as usize],
                );
                let down = Vec3::from_gba_color(
                    framebuffer_1[(y - 1).clamp(0, const { GBA_SCREEN_Y_I32 - 1 }) as usize]
                        [x as usize],
                );
                let right = Vec3::from_gba_color(
                    framebuffer_1[y as usize]
                        [(x + 1).clamp(0, const { GBA_SCREEN_X_I32 - 1 }) as usize],
                );
                let left = Vec3::from_gba_color(
                    framebuffer_1[y as usize]
                        [(x - 1).clamp(0, const { GBA_SCREEN_X_I32 - 1 }) as usize],
                );

                let mut avgcnt = 0;
                let mut tmpcolor = color - color;

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

    //
    // Basic acne removal, checks the 4 immediate neighbors and if 3
    //    are more than 5% different (by luminance) from the current pixel, replaces the current pixel with the average of neighbors
    //
    for y in 0..GBA_SCREEN_Y_I32 as usize {
        for x in 0..GBA_SCREEN_X_I32 as usize {
            unsafe {
                framebuffer_1[y][x] = bitmap.read_point(x as i32, y as i32);
            }
        }
    }
    for y in 0..GBA_SCREEN_Y_I32 {
        for x in 0..GBA_SCREEN_X_I32 {
            unsafe {
                let color = Vec3::from_gba_color(framebuffer_1[y as usize][x as usize]);
                let up = Vec3::from_gba_color(
                    framebuffer_1[(y + 1).clamp(0, const { GBA_SCREEN_Y_I32 - 1 }) as usize]
                        [x as usize],
                );
                let down = Vec3::from_gba_color(
                    framebuffer_1[(y - 1).clamp(0, const { GBA_SCREEN_Y_I32 - 1 }) as usize]
                        [x as usize],
                );
                let right = Vec3::from_gba_color(
                    framebuffer_1[y as usize]
                        [(x + 1).clamp(0, const { GBA_SCREEN_X_I32 - 1 }) as usize],
                );
                let left = Vec3::from_gba_color(
                    framebuffer_1[y as usize]
                        [(x - 1).clamp(0, const { GBA_SCREEN_X_I32 - 1 }) as usize],
                );

                let m = ((color.luma() - up.luma()).abs() > FixFlt::from_f32(0.05)) as u8
                    + ((color.luma() - down.luma()).abs() > FixFlt::from_f32(0.05)) as u8
                    + ((color.luma() - right.luma()).abs() > FixFlt::from_f32(0.05)) as u8
                    + ((color.luma() - left.luma()).abs() > FixFlt::from_f32(0.05)) as u8;

                if m >= 3 {
                    bitmap.draw_point(
                        x,
                        y,
                        ((up + down + right + left) * FixFlt::from(0.25)).to_gba_color(),
                    );
                }
            }
        }
    }
}

fn add_assign_f16_array(a: &mut [f16; 3], b: [f16; 3]) {
    for i in 0..3 {
        a[i] += b[i];
    }
}

#[link_section = ".ewram"] // Can hold a full rgb888 framebuffer or 555 framebuffer, perfect for both the low and high res denoisers
pub static mut DENOISING_WINDOW: [[[f16; 3]; 240]; 3] = [[[0f16; 3]; 240]; 3]; // one framebuffer row wide, 3 rows, 3 pixels per row

fn round_f16(x: f16) -> f16 {
    if x-x.next_down() < 0.5 {
        x.next_down()
    } else {
        x.next_up()
    }
}

pub fn hd_denoise(bitmap: &mut Bitmap3) {
    let bitmap_1 = as_rgb_view_mut();
    #[allow(static_mut_refs)]
    let window = unsafe { DENOISING_WINDOW.as_mut() };

    let mut color = [0u8; 3];
    let mut err_r = (color[0] & 0b00000111) as f16 / 16.0;
    let mut err_g = (color[1] & 0b00000111) as f16 / 16.0;
    let mut err_b = (color[2] & 0b00000111) as f16 / 16.0;

    for y in 0..GBA_SCREEN_Y_I32 {
        window[0] = window[1];
        window[1] = window[2];
        window[2] = [[0f16; 3]; 240];
        for x in 0..GBA_SCREEN_X_I32 {
            // # [0,    X,    7/16]
            // # [3/16, 5/16, 1/16]
            // Bayer extended, x is the current pixel
            color = [
                round_f16(f16::min(255.0, (bitmap_1[y as usize][x as usize][0] as f16 + window[0][x as usize][0] as f16))) as u8,
                round_f16(f16::min(255.0, (bitmap_1[y as usize][x as usize][1] as f16 + window[0][x as usize][1] as f16))) as u8,
                round_f16(f16::min(255.0, (bitmap_1[y as usize][x as usize][2] as f16 + window[0][x as usize][2] as f16))) as u8
            ];
            err_r = (color[0] & 0b00000111) as f16 / 16.0;
            err_g = (color[1] & 0b00000111) as f16 / 16.0;
            err_b = (color[2] & 0b00000111) as f16 / 16.0;

            if x + 1 < 240 {
                add_assign_f16_array(&mut window[0][(x + 1) as usize], [
                    // We need 555 color for the screen so the error is the 3 LSBs
                    // Using f16 as we are storing 1/64 the error
                    // one per different count since some are reused
                    err_r * 7f16,
                    err_g * 7f16,
                    err_b * 7f16,
                ])
            } // (2, 0)
            if x - 1 >= 0 {
                add_assign_f16_array(&mut window[1][(x - 1) as usize], [
                    // We need 555 color for the screen so the error is the 3 LSBs
                    // Using f16 as we are storing 1/64 the error
                    // one per different count since some are reused
                    err_r * 3f16,
                    err_g * 3f16,
                    err_b * 3f16,
                ])
            } // (0, 1)
            add_assign_f16_array(&mut window[1][x as usize], [
                // We need 555 color for the screen so the error is the 3 LSBs
                // Using f16 as we are storing 1/64 the error
                // one per different count since some are reused
                err_r * 5f16,
                err_g * 5f16,
                err_b * 5f16,
            ]);
            if x + 1 < 240 {
                add_assign_f16_array(&mut window[1][(x + 1) as usize], [
                    // We need 555 color for the screen so the error is the 3 LSBs
                    // Using f16 as we are storing 1/64 the error
                    // one per different count since some are reused
                    err_r,
                    err_g,
                    err_b,
                ])
            } // (2, 1)

            bitmap.draw_point(
                x,
                y,
                Vec3::new(
                    FixFlt::from_f32(
                        color[0] as f32 / 256.0,
                    ),
                    FixFlt::from_f32(
                        color[1] as f32 / 256.0,
                    ),
                    FixFlt::from_f32(
                        color[2] as f32 / 256.0,
                    ),
                )
                .to_gba_color(),
            );
        }
    }
}
