use fixed::types::I14F18;

pub type FixFlt = I14F18; // Fixed float, just a fixed point number, used to have a single type through the program if i need to adjust it

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Vec2 {
    pub x: FixFlt,
    pub y: FixFlt
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Vec3 {
    pub x: FixFlt,
    pub y: FixFlt,
    pub z: FixFlt
}