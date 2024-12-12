

use agb::timer::Timer;


use crate::rand::rand_double;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color {
            r: r / (255.0),
            g: g / (255.0),
            b: b / (255.0),
        }
    }
    pub fn new_01_range(r: f32, g: f32, b: f32) -> Color {
        Color { r: r, g: g, b: b }
    }
    pub fn rand(rng: &Timer) -> Color {
        Color {
            r: rand_double(rng),
            g: rand_double(rng),
            b: rand_double(rng),
        }
    }
}
