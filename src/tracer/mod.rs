mod raycolor;
mod sphere;

use agb::{display::bitmap3::Bitmap3, sound::mixer::Mixer};
use raycolor::ray_color;

use crate::{math::{ray::Ray, types::FixFlt, vec3::Vec3}, vars::{GBA_SCREEN_X, GBA_SCREEN_X_I32, GBA_SCREEN_Y, GBA_SCREEN_Y_I32}};

pub fn render(bitmap: &mut Bitmap3, viewport_height: FixFlt, viewport_width: FixFlt, focal_length: FixFlt, mixer: &mut Mixer) {
    let viewport_height_neg = -viewport_height;
    let pixel_height_y = viewport_height_neg / GBA_SCREEN_Y; // These two should be vectors, but i am doing the math manually
    let pixel_width_x = viewport_width / GBA_SCREEN_X;       // For speed and memory efficiency
    let camera_center = Vec3::new(
        0.0,
        0.0,
        0.0
    );
    let viewport_upper_left = Vec3::new( // Original code uses alot more clear code, but it would require 3x as much math
        camera_center.x - viewport_width * 0.5, // Multiply not divide to save cpu cycles
        camera_center.y - viewport_height_neg * 0.5,
        camera_center.z - focal_length
    );
    let pixel00_location = Vec3::new( // Same story here, just more efficient to do it this way :)
        viewport_upper_left.x + 0.5 * pixel_width_x,
        viewport_upper_left.y + 0.5 * pixel_height_y,
        focal_length
    );

    let mut pixel_center = pixel00_location;
    let mut ray = Ray::new(camera_center, pixel00_location-camera_center);
    for y in 0..GBA_SCREEN_Y_I32 {
        pixel_center.x = pixel00_location.x;
        pixel_center.y += pixel_height_y;
        for x in 0..GBA_SCREEN_X_I32 {
            pixel_center.x += pixel_width_x;
            ray.direction = pixel_center - camera_center;
            bitmap.draw_point(x, y, ray_color(&mut ray).to_gba_color());
            mixer.frame();
        }
    }
}
