

use agb::timer::Timer;
use agb_fixnum::{Num, num};

use crate::rand::rand_double;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: Num<i32, 16>,
    pub g: Num<i32, 16>,
    pub b: Num<i32, 16>,
}

impl Color {
    pub fn new(r: Num<i32, 16>, g: Num<i32, 16>, b: Num<i32, 16>) -> Color {
        Color {
            r: r / num!(255.0),
            g: g / num!(255.0),
            b: b / num!(255.0),
        }
    }
    pub fn new_01_range(r: Num<i32, 16>, g: Num<i32, 16>, b: Num<i32, 16>) -> Color {
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
