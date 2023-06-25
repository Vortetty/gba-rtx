

use agb::timer::Timer;
use agb_fixnum::{Num, num};
use fixed::types::I14F18;

use crate::rand::rand_double;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: I14F18,
    pub g: I14F18,
    pub b: I14F18,
}

impl Color {
    pub fn new(r: I14F18, g: I14F18, b: I14F18) -> Color {
        Color {
            r: r / I14F18::from_num(255.0),
            g: g / I14F18::from_num(255.0),
            b: b / I14F18::from_num(255.0),
        }
    }
    pub fn new_01_range(r: I14F18, g: I14F18, b: I14F18) -> Color {
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
