mod denoise;
mod interval;
mod material;
mod objects;
mod scene;

use agb::{
    display::bitmap3::{self, Bitmap3},
    dma::{self, Dma, Dmas},
    sound::mixer::Mixer,
};
use alloc::vec::Vec;
//use const_random::const_random;
use denoise::{as_rgb_view_mut, denoise, hd_denoise};
use material::MaterialManager;
use objects::sphere::Sphere;
use scene::Scene;
use tinyvec::ArrayVec;

use crate::{
    get_render_config::RenderConfig,
    math::{
        ray::Ray,
        types::{FixFlt, FRACTIONAL},
        vec3::Vec3,
    },
    vars::{GBA_SCREEN_X, GBA_SCREEN_X_I32, GBA_SCREEN_Y, GBA_SCREEN_Y_I32},
};


static mut MAT_MGR: MaterialManager = MaterialManager::new();
// Test scene, will be turned into it's own class later
static mut SCENE: Scene = Scene {
    spheres: vec![],
};

#[inline(never)]
#[link_section = ".iwram"]
pub fn render(
    bitmap: &mut Bitmap3,
    viewport_height: FixFlt,
    viewport_width: FixFlt,
    focal_length: FixFlt,
    settings: RenderConfig,
) {
    let viewport_height_neg = -viewport_height; // Need this negated for later calculations
    let pixel_height_y = viewport_height_neg / GBA_SCREEN_Y; // Calculate what fraction of the viewport each pixel is
    let pixel_width_x = viewport_width / GBA_SCREEN_X; // Calculate what fraction of the viewport each pixel is
    let camera_center = Vec3::new(
        // Camera position, will later be from the scene's settings.
        FixFlt::zero(),
        FixFlt::zero(),
        FixFlt::zero(),
    );
    let viewport_upper_left = Vec3::new(
        // Calculate the top left pixel's position
        camera_center.x - viewport_width * FixFlt::half_one(),
        camera_center.y - viewport_height_neg * FixFlt::half_one(),
        camera_center.z - focal_length,
    );
    let pixel00_location = Vec3::new(
        // Offset to the middle of the pixel
        viewport_upper_left.x + FixFlt::half_one() * pixel_width_x,
        viewport_upper_left.y + FixFlt::half_one() * pixel_height_y,
        viewport_upper_left.z,
    );

    let mut rng = FixFlt {
        inner: 0xC0FFEE // const_random!(i32),
    };

    #[allow(static_mut_refs)] // fuk u
    unsafe {
        SCENE.spheres.push(Sphere {
            // Center
            center: Vec3::new(FixFlt::zero(), FixFlt::zero(), FixFlt::from_f32(-1.2)),
            radius: FixFlt::half_one(),
            mat: MAT_MGR.add_lambertian(Vec3::new(
                FixFlt::from_f32(1.0),
                FixFlt::from_f32(0.3),
                FixFlt::from_f32(0.3),
            )),
        });
        SCENE.spheres.push(Sphere {
            // Left
            center: Vec3::new(
                FixFlt::from_f32(-1.0),
                FixFlt::zero(),
                FixFlt::from_f32(-1.0),
            ),
            radius: FixFlt::half_one(),
            mat: MAT_MGR.add_dielectric(
                Vec3::new(
                    FixFlt::from_f32(1.0),
                    FixFlt::from_f32(1.0),
                    FixFlt::from_f32(1.0),
                ),
                FixFlt::from_f32(1.5),
            ),
        });
        //SCENE.spheres.push(Sphere {
        //    // Left inner
        //    center: Vec3::new(
        //        FixFlt::from_f32(-1.0),
        //        FixFlt::zero(),
        //        FixFlt::from_f32(-1.0),
        //    ),
        //    radius: FixFlt::from_f32(0.4),
        //    mat: MAT_MGR.add_dielectric(
        //        Vec3::new(
        //            FixFlt::from_f32(1.0),
        //            FixFlt::from_f32(1.0),
        //            FixFlt::from_f32(1.0),
        //        ),
        //        FixFlt::from_f32(1.0/1.5),
        //    ),
        //});
        SCENE.spheres.push(Sphere {
            // Right
            center: Vec3::new(
                FixFlt::from_f32(1.0),
                FixFlt::zero(),
                FixFlt::from_f32(-1.0),
            ),
            radius: FixFlt::half_one(),
            mat: MAT_MGR.add_metal(
                Vec3::new(
                    FixFlt::from_f32(0.3),
                    FixFlt::from_f32(0.3),
                    FixFlt::from_f32(1.0),
                ),
                FixFlt::from_f32(1.0),
            ),
        });
        SCENE.spheres.push(Sphere {
            // Bottom
            center: Vec3::new(FixFlt::zero(), FixFlt::from_f32(-50.5), FixFlt::neg_one()),
            radius: FixFlt::from_i32(50),
            mat: MAT_MGR.add_lambertian(Vec3::new(
                FixFlt::from_f32(0.5),
                FixFlt::from_f32(0.5),
                FixFlt::from_f32(0.5),
            )),
        });
    }

    let mut precalc_offsets: Vec<Vec3> = vec![];
    let dims = closest_factors(settings.iters_per_pixel as i32);
    let xadd = pixel_width_x / dims.0;
    let yadd = pixel_height_y / dims.1;
    for x in 0..dims.0 {
        for y in 0..dims.1 {
            // Jittering within the cell boundaries
            let jitter_x = rng.next_rand_minmax(FixFlt::zero(), xadd);
            let jitter_y = rng.next_rand_minmax(FixFlt::zero(), yadd);

            unsafe {
                precalc_offsets.push(Vec3::new(
                    xadd * x + jitter_x,
                    yadd * y + jitter_y,
                    FixFlt::zero(),
                ));
            }
        }
    }

    let mut pixel_center = pixel00_location;
    let mut ray = Ray::new(camera_center, pixel00_location - camera_center);
    let mut out_color: Vec3 = Vec3::new(FixFlt::zero(), FixFlt::zero(), FixFlt::zero());
    let iters_recip = FixFlt::from(settings.iters_per_pixel).recip();
    if settings.hd_mode {
        let bitmap_1 = as_rgb_view_mut();
        #[allow(static_mut_refs)]
        let window = unsafe { DENOISING_WINDOW.as_mut() };
        let mut color = [0u8; 3];
        let mut err_r = (color[0] & 0b00000111) as f16 / 16.0;
        let mut err_g = (color[1] & 0b00000111) as f16 / 16.0;
        let mut err_b = (color[2] & 0b00000111) as f16 / 16.0;

        for y in 0..GBA_SCREEN_Y_I32 {
            pixel_center.x = pixel00_location.x;
            pixel_center.y += pixel_height_y;

            // Shift window down a row and move in blank info
            window[0] = window[1];
            window[1] = window[2];
            window[2] = [[0f16; 3]; 240];

            for x in 0..GBA_SCREEN_X_I32 {
                pixel_center.x += pixel_width_x;
                ray.direction = pixel_center - camera_center;
                out_color.x.inner = 0;
                out_color.y.inner = 0;
                out_color.z.inner = 0;
                for i in precalc_offsets.iter() {
                    let mut tmpray = ray;
                    tmpray.direction = tmpray.direction + *i;
                    out_color = out_color + unsafe{SCENE.ray_color(&mut tmpray, &mut rng, &settings, &MAT_MGR)};
                }
                color = (out_color * iters_recip).to_888_color();

                color = [
                    round_f16(f16::min(255.0, (color[0] as f16 + window[0][x as usize][0] as f16))) as u8,
                    round_f16(f16::min(255.0, (color[1] as f16 + window[0][x as usize][1] as f16))) as u8,
                    round_f16(f16::min(255.0, (color[2] as f16 + window[0][x as usize][2] as f16))) as u8
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
                        err_r * 3f16,
                        err_g * 3f16,
                        err_b * 3f16,
                    ])
                } // (0, 1)
                add_assign_f16_array(&mut window[1][x as usize], [
                    err_r * 5f16,
                    err_g * 5f16,
                    err_b * 5f16,
                ]); // (1, 1)
                if x + 1 < 240 {
                    add_assign_f16_array(&mut window[1][(x + 1) as usize], [
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

        //hd_denoise(bitmap); // Change no.3 from the normal loop
    } else {
        for y in 0..GBA_SCREEN_Y_I32 {
            pixel_center.x = pixel00_location.x;
            pixel_center.y += pixel_height_y;
            for x in 0..GBA_SCREEN_X_I32 {
                pixel_center.x += pixel_width_x;
                ray.direction = pixel_center - camera_center;
                out_color.x.inner = 0;
                out_color.y.inner = 0;
                out_color.z.inner = 0;
                for i in precalc_offsets.iter() {
                    let mut tmpray = ray;
                    tmpray.direction = tmpray.direction + *i;
                    out_color =
                        out_color + unsafe{SCENE.ray_color(&mut tmpray, &mut rng, &settings, &MAT_MGR)};
                }
                bitmap.draw_point(
                    x as i32,
                    y as i32,
                    (out_color * iters_recip).to_gba_color(),
                );
            }
        }
    }

    denoise(bitmap);
}


#[link_section = ".ewram"] // Can hold a full rgb888 framebuffer or 555 framebuffer, perfect for both the low and high res denoisers
pub static mut DENOISING_WINDOW: [[[f16; 3]; 240]; 3] = [[[0f16; 3]; 240]; 3]; // one framebuffer row wide, 3 rows, 3 pixels per row

fn add_assign_f16_array(a: &mut [f16; 3], b: [f16; 3]) {
    for i in 0..3 {
        a[i] += b[i];
    }
}

fn round_f16(x: f16) -> f16 {
    if x-x.next_down() < 0.5 {
        x.next_down()
    } else {
        x.next_up()
    }
}

fn closest_factors(n: i32) -> (i32, i32) {
    let sqrt = FixFlt::from_i32(n).sqrt().inner >> FRACTIONAL; // Compute the integer square root
    if sqrt * sqrt == n {
        return (sqrt, sqrt); // Return the square root if it's a perfect square
    }

    let mut closest = (1, n); // Initialize with the trivial factors
    let mut min_diff = n; // Start with the maximum possible difference

    for i in 1..=sqrt {
        if n % i == 0 {
            let pair = (i, n / i); // i is a factor, and n / i is its pair
            let diff = (pair.1 as i32 - pair.0 as i32).abs(); // Calculate the difference
            if diff < min_diff as i32 {
                closest = pair; // Update the closest factors
                min_diff = diff as i32;
            }
        }
    }

    closest
}