use core::ops::{Add, Div, Mul, Neg, Sub};

use super::types::{FixFlt, FixFltOnce, FRACTIONAL};

#[derive(Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: FixFlt,
    pub y: FixFlt,
    pub z: FixFlt,

    length_square: FixFltOnce,
    length: FixFltOnce,
}

macro_rules! impl_ops {
    ($trait:ident, $method:ident, $op:tt) => {
        // Element-wise operations for Vec3 and Vec3
        impl $trait<Self> for Vec3 {
            type Output = Self;

            #[inline]
            #[link_section = ".iwram"]
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

            #[inline]
            #[link_section = ".iwram"]
            fn $method(self, rhs: FixFlt) -> Self {
                Self::new(
                    self.x $op rhs,
                    self.y $op rhs,
                    self.z $op rhs
                )
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
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Vec3 {
    pub const fn new(x: FixFlt, y: FixFlt, z: FixFlt) -> Self {
        Self {
            x,
            y,
            z,
            length: FixFltOnce::new(),
            length_square: FixFltOnce::new(),
        }
    }

    pub fn length_squared(&mut self) -> FixFlt {
        self.length_square
            .init_and_get(|| -> FixFlt { self.x * self.x + self.y * self.y + self.z * self.z })
    }
    pub fn length(&mut self) -> FixFlt {
        let lensqr = self.length_squared();
        self.length.init_and_get(|| -> FixFlt { lensqr.sqrt() })
    }

    pub fn dot_prod(&self, rhs: &Self) -> FixFlt {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn cross_prod(&self, rhs: &Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
    pub fn unit_vec(&mut self) -> Self {
        *self * self.length_squared().invsqrt()
        //*self / self.length()
    }
    pub fn random_unit_vec(rng: &mut FixFlt) -> Self {
        loop {
            let mut a = Self::new(
                rng.next_rand_minmax(FixFlt::from_f32(-1.0), FixFlt::from_f32(1.0)),
                rng.next_rand_minmax(FixFlt::from_f32(-1.0), FixFlt::from_f32(1.0)),
                rng.next_rand_minmax(FixFlt::from_f32(-1.0), FixFlt::from_f32(1.0)),
            );
            let b = a.length_squared();
            if const { FixFlt { inner: 0x8 } } < b && b <= FixFlt::one() {
                return a * a.length();
            }
        }
    }

    pub fn reset_cached(&mut self) {
        self.length = FixFltOnce::new();
        self.length_square = FixFltOnce::new();
    }

    pub fn random(rng: &mut FixFlt) -> Vec3 {
        Self::new(
            rng.next_rand_frac(),
            rng.next_rand_frac(),
            rng.next_rand_frac(),
        )
    }
    pub fn random_minmax(rng: &mut FixFlt, min: FixFlt, max: FixFlt) -> Vec3 {
        Self::new(
            rng.next_rand_minmax(FixFlt::from_f32(-1.0), FixFlt::from_f32(1.0)),
            rng.next_rand_minmax(FixFlt::from_f32(-1.0), FixFlt::from_f32(1.0)),
            rng.next_rand_minmax(FixFlt::from_f32(-1.0), FixFlt::from_f32(1.0)),
        )
    }
    pub fn random_hemisphere(rng: &mut FixFlt, normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vec(rng);
        if on_unit_sphere.dot_prod(normal) > FixFlt::zero() {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
    pub fn near_zero(&self) -> bool {
        (self.x.inner < 0b000000000000_0000000000000001000)
            && (self.y.inner < 0b000000000000_0000000000000001000)
            && (self.z.inner < 0b000000000000_0000000000000001000)
    }
    pub fn reflect(&self, normal: &Vec3) -> Self {
        *self - *normal*FixFlt::from_i32(2)*self.dot_prod(normal)
    }
    pub fn refract(&self, normal: &Vec3, etai_over_etat: FixFlt, cos_theta: FixFlt) -> Self {
        let mut r_out_perp = (*self + *normal * cos_theta) * etai_over_etat;
        let r_out_parallel = *normal * -((FixFlt::one() - r_out_perp.length_squared()).abs().sqrt());
        return r_out_perp + r_out_parallel;
    }

    //
    // COLOR FUNCS
    //
    pub fn to_gba_color(&self) -> u16 {
        ((self.z.to_bits() >> const { FRACTIONAL - 5 }) as u16) << 10
            | ((self.y.to_bits() >> const { FRACTIONAL - 5 }) as u16) << 5
            | ((self.x.to_bits() >> const { FRACTIONAL - 5 }) as u16)
    }
    pub fn to_888_color(&self) -> [u8; 3] {
        [
            (((self.x.to_bits() >> const { FRACTIONAL - 8 }) & 0b11111111) as u8),
            (((self.y.to_bits() >> const { FRACTIONAL - 8 }) & 0b11111111) as u8),
            (((self.z.to_bits() >> const { FRACTIONAL - 8 }) & 0b11111111) as u8)
        ]
    }
    pub fn from_gba_color(rhs: u16) -> Self {
        Self::new(
            FixFlt {
                inner: ((rhs & 0b11111) as i32) << const { FRACTIONAL - 5 },
            },
            FixFlt {
                inner: (((rhs >> 5) & 0b11111) as i32) << const { FRACTIONAL - 5 },
            },
            FixFlt {
                inner: (((rhs >> 10) & 0b11111) as i32) << const { FRACTIONAL - 5 },
            },
        )
    }
    pub fn luma(&self) -> FixFlt {
        // Based on the rec 709 standard
        FixFlt::from_f32(0.2126) * self.x
            + FixFlt::from_f32(0.7152) * self.y
            + FixFlt::from_f32(0.0722) * self.z
    }
}
