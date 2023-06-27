

use agb::timer::Timer;
use agb_fixnum::{Num, num};
use fixed::types::I34F30;

use crate::rand::rand_double;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: I34F30,
    pub g: I34F30,
    pub b: I34F30,
}

impl Color {
    pub fn new(r: I34F30, g: I34F30, b: I34F30) -> Color {
        Color {
            r: r / I34F30::from_num(255.0),
            g: g / I34F30::from_num(255.0),
            b: b / I34F30::from_num(255.0),
        }
    }
    pub fn new_01_range(r: I34F30, g: I34F30, b: I34F30) -> Color {
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
