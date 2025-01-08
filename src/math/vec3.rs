use core::ops::{Add, Div, Mul, Sub};

use super::types::FixFlt;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Vec3 {
    pub x: FixFlt,
    pub y: FixFlt,
    pub z: FixFlt
}

struct Color {
    r: FixFlt,
    g: FixFlt,
    b: FixFlt,
}

impl From<Vec3> for Color {
    fn from(vec: Vec3) -> Self {
        Self {
            r: vec.x,
            g: vec.y,
            b: vec.z,
        }
    }
}

impl From<Color> for Vec3 {
    fn from(color: Color) -> Self {
        Self {
            x: color.r,
            y: color.g,
            z: color.b,
        }
    }
}
macro_rules! impl_ops {
    ($trait:ident, $method:ident, $op:tt) => {
        // Element-wise operations for Vec3 and Vec3
        impl $trait<Self> for Vec3 {
            type Output = Self;

            #[inline(always)]
            fn $method(self, rhs: Self) -> Self {
                Self {
                    x: self.x $op rhs.x,
                    y: self.y $op rhs.y,
                    z: self.z $op rhs.z,
                }
            }
        }

        // Scalar operations for Vec3 and FixFlt
        impl $trait<FixFlt> for Vec3 {
            type Output = Self;

            #[inline(always)]
            fn $method(self, rhs: FixFlt) -> Self {
                Self {
                    x: self.x $op rhs,
                    y: self.y $op rhs,
                    z: self.z $op rhs,
                }
            }
        }

        // Element-wise operations for Color and Color
        impl $trait<Self> for Color {
            type Output = Self;

            #[inline(always)]
            fn $method(self, rhs: Self) -> Self {
                Self {
                    r: self.r $op rhs.r,
                    g: self.g $op rhs.g,
                    b: self.b $op rhs.b,
                }
            }
        }

        // Scalar operations for Color and FixFlt
        impl $trait<FixFlt> for Color {
            type Output = Self;

            #[inline(always)]
            fn $method(self, rhs: FixFlt) -> Self {
                Self {
                    r: self.r $op rhs,
                    g: self.g $op rhs,
                    b: self.b $op rhs,
                }
            }
        }
    };
}

// Macro invocation for the arithmetic traits
impl_ops!(Add, add, +);
impl_ops!(Sub, sub, -);
impl_ops!(Mul, mul, *);
impl_ops!(Div, div, /);