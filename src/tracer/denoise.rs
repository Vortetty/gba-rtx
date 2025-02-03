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
    unsafe {
        &mut *(FRAMEBUFFER_1_STATIC.0.as_mut_ptr() as *mut [[u16; 240]; 160])
    }
}

// Safe access to the u8 view (160 * 3 bytes per row)
#[allow(static_mut_refs)]
pub fn as_rgb_view_mut() -> &'static mut [[[u8; 3]; 240]; 160] {
    unsafe {
        &mut *(FRAMEBUFFER_1_STATIC.0.as_mut_ptr() as *mut [[[u8; 3]; 240]; 160])
    }
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

pub fn hd_denoise(bitmap: &mut Bitmap3) {
    let bitmap_1 = as_rgb_view_mut();
    #[allow(static_mut_refs)]
    let window = unsafe { DENOISING_WINDOW.as_mut() };


    let mut color = [0u8; 3];
    let mut error10 = [0f16; 3];
    let mut error6 = [0f16; 3];
    let mut error5 = [0f16; 3];
    let mut error4 = [0f16; 3];
    let mut error3 = [0f16; 3];
    let mut error2 = [0f16; 3];

    for y in 0..GBA_SCREEN_Y_I32 {
        window[0] = window[1];
        window[1] = window[2];
        window[2] = [[0f16; 3]; 240];
        for x in 0..GBA_SCREEN_X_I32 {
            // # [0,    0,    X,     10/64,  5/64]
            // # [3/64, 6/64, 10/64, 6/64,   3/64]
            // # [2/64, 3/64, 4/64,  3/64,   2/64]
            // Bayer extended, x is the current pixel
            color = bitmap_1[y as usize][x as usize];
            error10 = [ // We need 555 color for the screen so the error is the 3 LSBs
                        // Using f16 as we are storing 1/64 the error
                        // one per different count since some are reused
                (color[0] & 0b00000111) as f16 / 64f16 * 10f16,
                (color[1] & 0b00000111) as f16 / 64f16 * 10f16,
                (color[2] & 0b00000111) as f16 / 64f16 * 10f16
            ];
            error6 = [
                (color[0] & 0b00000111) as f16 / 64f16 * 6f16,
                (color[1] & 0b00000111) as f16 / 64f16 * 6f16,
                (color[2] & 0b00000111) as f16 / 64f16 * 6f16
            ];
            error5 = [
                (color[0] & 0b00000111) as f16 / 64f16 * 5f16,
                (color[1] & 0b00000111) as f16 / 64f16 * 5f16,
                (color[2] & 0b00000111) as f16 / 64f16 * 5f16
            ];
            error4 = [
                (color[0] & 0b00000111) as f16 / 64f16 * 4f16,
                (color[1] & 0b00000111) as f16 / 64f16 * 4f16,
                (color[2] & 0b00000111) as f16 / 64f16 * 4f16
            ];
            error3 = [
                (color[0] & 0b00000111) as f16 / 64f16 * 3f16,
                (color[1] & 0b00000111) as f16 / 64f16 * 3f16,
                (color[2] & 0b00000111) as f16 / 64f16 * 3f16
            ];
            error2 = [
                (color[0] & 0b00000111) as f16 / 64f16 * 2f16,
                (color[1] & 0b00000111) as f16 / 64f16 * 2f16,
                (color[2] & 0b00000111) as f16 / 64f16 * 2f16
            ];
            // # [0,    0,    X,     10/64,  5/64]
            // # [3/64, 6/64, 10/64, 6/64,   3/64]
            // # [2/64, 3/64, 4/64,  3/64,   2/64]
            // Bayer extended, x is the current pixel
            // go through and handle each pixel, manually unrolled loop.
            // skip the first 3 since you add nothing to them
            // and we don't have to check y since the error window will always be the right size :)
            if x+1 < 240 { add_assign_f16_array(&mut window[0][(x+1) as usize], error10) } // (3, 0)
            if x+2 < 240 { add_assign_f16_array(&mut window[0][(x+2) as usize], error5) } // (4, 0)

            if x-2 >= 0 { add_assign_f16_array(&mut window[1][(x-2) as usize], error3) } // (0, 1)
            if x-1 >= 0 { add_assign_f16_array(&mut window[1][(x-1) as usize], error6) } // (1, 1)
            add_assign_f16_array(&mut window[1][x as usize], error10); // (2, 1)
            if x+1 < 240 { add_assign_f16_array(&mut window[1][(x+1) as usize], error6) } // (3, 1)
            if x+2 < 240 { add_assign_f16_array(&mut window[1][(x+2) as usize], error3) } // (4, 1)

            if x-2 >= 0 { add_assign_f16_array(&mut window[2][(x-2) as usize], error2) } // (0, 2)
            if x-1 >= 0 { add_assign_f16_array(&mut window[2][(x-1) as usize], error3) } // (1, 2)
            add_assign_f16_array(&mut window[2][x as usize], error4); // (2, 1)
            if x+1 < 240 { add_assign_f16_array(&mut window[2][(x+1) as usize], error3) } // (3, 2)
            if x+2 < 240 { add_assign_f16_array(&mut window[2][(x+2) as usize], error2) } // (4, 2)

            bitmap.draw_point(x, y,
                Vec3::new(
                    FixFlt::from_f32(f32::min(255.0, (color[0] as f32 + window[0][x as usize][0] as f32))/256.0),
                    FixFlt::from_f32(f32::min(255.0, (color[1] as f32 + window[0][x as usize][1] as f32))/256.0),
                    FixFlt::from_f32(f32::min(255.0, (color[2] as f32 + window[0][x as usize][2] as f32))/256.0)
                ).to_gba_color()
            );
        }
    }
}