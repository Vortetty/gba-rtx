use super::types::FixFlt;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Vec3 {
    pub x: FixFlt,
    pub y: FixFlt,
    pub z: FixFlt
}