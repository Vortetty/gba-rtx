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
use arrayvec::ArrayVec;
use const_random::const_random;
use denoise::{as_rgb_view_mut, denoise, hd_denoise};
use material::MaterialManager;
use objects::sphere::Sphere;
use scene::Scene;

use crate::{
    get_render_config::RenderConfig,
    math::{
        ray::Ray,
        types::{FixFlt, FRACTIONAL},
        vec3::Vec3,
    },
    vars::{GBA_SCREEN_X, GBA_SCREEN_X_I32, GBA_SCREEN_Y, GBA_SCREEN_Y_I32},
};

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

    let mut mat_mgr = MaterialManager::new();
    // Test scene, will be turned into it's own class later
    let mut scene = Scene {
        spheres: vec![
            Sphere {
                // Center
                center: Vec3::new(FixFlt::zero(), FixFlt::zero(), FixFlt::from_f32(-1.2)),
                radius: FixFlt::half_one(),
                mat: mat_mgr.add_lambertian(Vec3::new(
                    FixFlt::from_f32(1.0),
                    FixFlt::from_f32(0.3),
                    FixFlt::from_f32(0.3),
                )),
            },
            Sphere {
                // Left
                center: Vec3::new(
                    FixFlt::from_f32(-1.0),
                    FixFlt::zero(),
                    FixFlt::from_f32(-1.0),
                ),
                radius: FixFlt::half_one(),
                mat: mat_mgr.add_metal(
                    Vec3::new(
                        FixFlt::from_f32(0.3),
                        FixFlt::from_f32(1.0),
                        FixFlt::from_f32(0.3),
                    ),
                    FixFlt::from_f32(0.2),
                ),
            },
            Sphere {
                // Right
                center: Vec3::new(
                    FixFlt::from_f32(1.0),
                    FixFlt::zero(),
                    FixFlt::from_f32(-1.0),
                ),
                radius: FixFlt::half_one(),
                mat: mat_mgr.add_metal(
                    Vec3::new(
                        FixFlt::from_f32(0.3),
                        FixFlt::from_f32(0.3),
                        FixFlt::from_f32(1.0),
                    ),
                    FixFlt::from_f32(1.0),
                ),
            },
            Sphere {
                // Bottom
                center: Vec3::new(FixFlt::zero(), FixFlt::from_f32(-50.5), FixFlt::neg_one()),
                radius: FixFlt::from_i32(50),
                mat: mat_mgr.add_lambertian(Vec3::new(
                    FixFlt::from_f32(0.9),
                    FixFlt::from_f32(0.9),
                    FixFlt::from_f32(0.9),
                )),
            },
        ],
    };

    let mut rng = FixFlt {
        inner: const_random!(i32),
    };

    let mut precalc_offsets: ArrayVec<Vec3, 256> = ArrayVec::<Vec3, 256>::new();
    let dims = closest_factors(settings.iters_per_pixel as i32);
    let xadd = pixel_width_x / dims.0;
    let yadd = pixel_height_y / dims.1;
    for x in 0..dims.0 {
        for y in 0..dims.1 {
            // Jittering within the cell boundaries
            let jitter_x = rng.next_rand_minmax(FixFlt::zero(), xadd);
            let jitter_y = rng.next_rand_minmax(FixFlt::zero(), yadd);

            unsafe {
                precalc_offsets.push_unchecked(Vec3::new(
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
    if settings.hd_mode {
        let rgbf16_bitmap = as_rgb_view_mut();  // Change no.1 from the normal loop
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
                        out_color + scene.ray_color(&mut tmpray, &mut rng, &settings, &mat_mgr);
                }
                out_color = out_color * FixFlt::from(settings.iters_per_pixel).recip();
                bitmap.draw_point( // We should probably not display this but... seeing the scanlines is part of the magic so fuck speed amirite?
                    x as i32,
                    y as i32,
                    out_color.to_gba_color(),
                );
                rgbf16_bitmap[y as usize][x as usize] = out_color.to_888_color(); // Change no.2 from the normal loop
            }
        }

        hd_denoise(bitmap); // Change no.3 from the normal loop
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
                        out_color + scene.ray_color(&mut tmpray, &mut rng, &settings, &mat_mgr);
                }
                bitmap.draw_point(
                    x as i32,
                    y as i32,
                    (out_color * FixFlt::from(settings.iters_per_pixel).recip()).to_gba_color(),
                );
            }
        }

        denoise(bitmap);
    }
}
