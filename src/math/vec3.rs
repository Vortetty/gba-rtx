use core::ops::{Add, Div, Mul, Neg, Sub};


use super::types::{FixFlt, FixFltOnce, FRACTIONAL};

#[derive(Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: FixFlt,
    pub y: FixFlt,
    pub z: FixFlt,

    length_square: FixFltOnce,
    length: FixFltOnce
}

#[derive(Debug)]
pub struct Color {
    pub r: FixFlt,
    pub g: FixFlt,
    pub b: FixFlt,
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
        Self::new(
            color.r,
            color.g,
            color.b
        )
    }
}
macro_rules! impl_ops {
    ($trait:ident, $method:ident, $op:tt) => {
        // Element-wise operations for Vec3 and Vec3
        impl $trait<Self> for Vec3 {
            type Output = Self;

            #[inline(always)]
            fn $method(self, rhs: Self) -> Self {
                Self::new(
                    self.x $op rhs.x,
                    self.y $op rhs.y,
                    self.z $op rhs.z
                )
            }
        }

        // Scalar operations for Vec3 and FixFlt
        impl $trait<FixFlt> for Vec3 {
            type Output = Self;

            #[inline(always)]
            fn $method(self, rhs: FixFlt) -> Self {
                Self::new(
                    self.x $op rhs,
                    self.y $op rhs,
                    self.z $op rhs
                )
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

// Basic arithmetic!
impl_ops!(Add, add, +);
impl_ops!(Sub, sub, -);
impl_ops!(Mul, mul, *);
impl_ops!(Div, div, /);

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::new(
            -self.x,
            -self.y,
            -self.z
        )
    }
}

impl Vec3 {
    #[inline(always)]
    pub const fn new(x: FixFlt, y: FixFlt, z: FixFlt) -> Self {
        Self {
            x,
            y,
            z,
            length: FixFltOnce::new(),
            length_square: FixFltOnce::new()
        }
    }

    #[inline(always)]
    pub fn length_squared(&mut self) -> FixFlt {
        self.length_square.init_and_get(|| -> FixFlt {
            self.x*self.x + self.y*self.y + self.z*self.z
        })
    }
    #[inline(always)]
    pub fn length(&mut self) -> FixFlt {
        let lensqr = self.length_squared();
        self.length.init_and_get(|| -> FixFlt {
            lensqr.sqrt()
        })
    }

    #[inline(always)]
    pub fn dot_prod(&self, rhs: &Self) -> FixFlt {
        self.x * rhs.x +
        self.y * rhs.y +
        self.z * rhs.z
    }
    #[inline(always)]
    pub fn cross_prod(&self, rhs: &Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x
        )
    }
    #[inline(always)]
    pub fn unit_vec(&mut self) -> Self {
        *self / self.length()
    }
    #[inline(always)]
    pub fn random_unit_vec(rng: &mut FixFlt) -> Self {
        loop {
            let mut a = Self::new(
                rng.next_rand_minmax(FixFlt::from_f32(-1.0), FixFlt::from_f32(1.0)),
                rng.next_rand_minmax(FixFlt::from_f32(-1.0), FixFlt::from_f32(1.0)),
                rng.next_rand_minmax(FixFlt::from_f32(-1.0), FixFlt::from_f32(1.0))
            );
            let b = a.length_squared();
            if const {FixFlt {inner: 0x2}} < b && b <= FixFlt::one() {
                return a / a.length();
            }
        }
    }

    #[inline(always)]
    pub fn reset_cached(&mut self) {
        self.length = FixFltOnce::new();
        self.length_square = FixFltOnce::new();
    }

    #[inline(always)]
    pub fn random(rng: &mut FixFlt) -> Vec3 {
        Self::new(
            rng.next_rand_frac(),
            rng.next_rand_frac(),
            rng.next_rand_frac()
        )
    }
    #[inline(always)]
    pub fn random_minmax(rng: &mut FixFlt, min: FixFlt, max: FixFlt) -> Vec3 {
        Self::new(
            rng.next_rand_minmax(FixFlt::from_f32(-1.0), FixFlt::from_f32(1.0)),
            rng.next_rand_minmax(FixFlt::from_f32(-1.0), FixFlt::from_f32(1.0)),
            rng.next_rand_minmax(FixFlt::from_f32(-1.0), FixFlt::from_f32(1.0))
        )
    }
    #[inline(always)]
    pub fn random_hemisphere(rng: &mut FixFlt, normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vec(rng);
        if on_unit_sphere.dot_prod(normal) > FixFlt::zero() {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
}

impl Color {
    #[inline(always)]
    pub fn to_gba_color(&self) -> u16 {
        ((self.b.to_bits() >> const {FRACTIONAL-5}) as u16) << 10 |
        ((self.g.to_bits() >> const {FRACTIONAL-5}) as u16) << 5 |
        ((self.r.to_bits() >> const {FRACTIONAL-5}) as u16)
    }
    #[inline(always)]
    pub const fn new(r: FixFlt, g: FixFlt, b: FixFlt) -> Self {
        Self {
            r,
            g,
            b
        }
    }
}
