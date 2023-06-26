

use agb::timer::Timer;
use agb_fixnum::{Num, num};
use fixed::types::I20F12;

use crate::rand::rand_double;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: I20F12,
    pub g: I20F12,
    pub b: I20F12,
}

impl Color {
    pub fn new(r: I20F12, g: I20F12, b: I20F12) -> Color {
        Color {
            r: r / I20F12::from_num(255.0),
            g: g / I20F12::from_num(255.0),
            b: b / I20F12::from_num(255.0),
        }
    }
    pub fn new_01_range(r: I20F12, g: I20F12, b: I20F12) -> Color {
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
