use core::ops::{Add, Div, Mul, Sub};

use super::types::FixFlt;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Vec2 {
    pub x: FixFlt,
    pub y: FixFlt
}

struct Ray {
    yaw: FixFlt,
    pitch: FixFlt,
}

macro_rules! impl_ops {
    ($trait:ident, $method:ident, $op:tt) => {
        // Element-wise operations for Vec2 (Vect) and Vec2
        impl $trait<Self> for Vec2 {
            type Output = Self;

            #[inline(always)]
            fn $method(self, rhs: Self) -> Self {
                Self {
                    x: self.x $op rhs.x,
                    y: self.y $op rhs.y,
                }
            }
        }

        // Scalar operations for Vec2 (Vect) and FixFlt
        impl $trait<FixFlt> for Vec2 {
            type Output = Self;

            #[inline(always)]
            fn $method(self, rhs: FixFlt) -> Self {
                Self {
                    x: self.x $op rhs,
                    y: self.y $op rhs,
                }
            }
        }

        // Element-wise operations for Ray and Ray
        impl $trait<Self> for Ray {
            type Output = Self;

            #[inline(always)]
            fn $method(self, rhs: Self) -> Self {
                Self {
                    yaw: self.yaw $op rhs.yaw,
                    pitch: self.pitch $op rhs.pitch,
                }
            }
        }

        // Scalar operations for Ray and FixFlt
        impl $trait<FixFlt> for Ray {
            type Output = Self;

            #[inline(always)]
            fn $method(self, rhs: FixFlt) -> Self {
                Self {
                    yaw: self.yaw $op rhs,
                    pitch: self.pitch $op rhs,
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

