mod scene;
mod objects;
mod interval;

use agb::{display::bitmap3::Bitmap3, sound::mixer::Mixer};
use objects::sphere::Sphere;
use scene::Scene;

use crate::{math::{ray::Ray, types::FixFlt, vec3::Vec3}, vars::{GBA_SCREEN_X, GBA_SCREEN_X_I32, GBA_SCREEN_Y, GBA_SCREEN_Y_I32}};

#[link_section = ".iwram"]
pub fn render(bitmap: &mut Bitmap3, viewport_height: FixFlt, viewport_width: FixFlt, focal_length: FixFlt, mixer: &mut Mixer) {
    let viewport_height_neg = -viewport_height;
    let pixel_height_y = viewport_height_neg / GBA_SCREEN_Y; // These two should be vectors, but i am doing the math manually
    let pixel_width_x = viewport_width / GBA_SCREEN_X;       // For speed and memory efficiency
    let camera_center = Vec3::new(
        FixFlt::zero(),
        FixFlt::zero(),
        FixFlt::zero()
    );
    let viewport_upper_left = Vec3::new( // Original code uses alot more clear code, but it would require 3x as much math
        camera_center.x - viewport_width * FixFlt::half_one(), // Multiply not divide to save cpu cycles
        camera_center.y - viewport_height_neg * FixFlt::half_one(),
        camera_center.z - focal_length
    );
    let pixel00_location = Vec3::new( // Same story here, just more efficient to do it this way :)
        viewport_upper_left.x + FixFlt::half_one() * pixel_width_x,
        viewport_upper_left.y + FixFlt::half_one() * pixel_height_y,
        viewport_upper_left.z + FixFlt::half_one()
    );

    let mut scene = Scene {
        spheres: vec![
            Sphere {
                center: Vec3::new(
                    FixFlt::zero(),
                    FixFlt::zero(),
                    FixFlt::neg_one()
                ),
                radius: FixFlt::half_one()
            },
            Sphere {
                center: Vec3::new(
                    FixFlt::zero(),
                    FixFlt::from_f32(-50.5),
                    FixFlt::neg_one()
                ),
                radius: FixFlt::from_i32(50)
            }
        ]
    };

    let mut pixel_center = pixel00_location;
    let mut ray = Ray::new(camera_center, pixel00_location-camera_center);
    for y in 0..GBA_SCREEN_Y_I32 {
        pixel_center.x = pixel00_location.x;
        pixel_center.y += pixel_height_y;
        for x in 0..GBA_SCREEN_X_I32 {

            pixel_center.x += pixel_width_x;
            ray.direction = pixel_center - camera_center;
            bitmap.draw_point(x as i32, y as i32, scene.ray_color(&mut ray).to_gba_color());
            mixer.frame();
        }
    }
}
