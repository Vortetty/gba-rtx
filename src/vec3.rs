use core::{ops::{Add, Mul, Div, Sub, Neg, Shl, Shr}, i16::MIN};

use agb::timer::Timer;
use agb_fixnum::{Num, num};
use fixed::types::I34F30;

use crate::{rand::{rand_double, rand_double_range}, trig_num::TrigNum};

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: I34F30,
    pub y: I34F30,
    pub z: I34F30,
}
impl Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, _rhs: Self) -> Self {
        Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}
impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, _rhs: Self) -> Self {
        Self {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        }
    }
}
impl Mul<i64> for Vec3 {
    type Output = Self;
    fn mul(self, _rhs: i64) -> Self {
        Self {
            x: self.x * I34F30::from_num(_rhs as i64),
            y: self.y * I34F30::from_num(_rhs as i64),
            z: self.z * I34F30::from_num(_rhs as i64),
        }
    }
}
impl Mul<I34F30> for Vec3 {
    type Output = Self;
    fn mul(self, _rhs: I34F30) -> Self {
        Self {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}
impl Div<Vec3> for Vec3 {
    type Output = Self;
    fn div(self, _rhs: Self) -> Self {
        Self {
            x: self.x / _rhs.x,
            y: self.y / _rhs.y,
            z: self.z / _rhs.z,
        }
    }
}
impl Div<i64> for Vec3 {
    type Output = Self;
    fn div(self, _rhs: i64) -> Self {
        Self {
            x: self.x / I34F30::from_num(_rhs as i64),
            y: self.y / I34F30::from_num(_rhs as i64),
            z: self.z / I34F30::from_num(_rhs as i64),
        }
    }
}
impl Div<I34F30> for Vec3 {
    type Output = Self;
    fn div(self, _rhs: I34F30) -> Self {
        Self {
            x: self.x.checked_div(_rhs).unwrap_or_else(|| I34F30::ZERO),
            y: self.y.checked_div(_rhs).unwrap_or_else(|| I34F30::ZERO),
            z: self.z.checked_div(_rhs).unwrap_or_else(|| I34F30::ZERO),
        }
    }
}
impl Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, _rhs: Self) -> Self {
        Self {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}
impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: self.x.checked_neg().unwrap(),
            y: self.y.checked_neg().unwrap(),
            z: self.z.checked_neg().unwrap(),
        }
    }
}

impl Mul<Vec3> for i64 {
    type Output = Vec3;
    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: _rhs.x * I34F30::from_num(self as i64),
            y: _rhs.y * I34F30::from_num(self as i64),
            z: _rhs.z * I34F30::from_num(self as i64),
        }
    }
}
impl Mul<Vec3> for I34F30 {
    type Output = Vec3;
    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: _rhs.x * self,
            y: _rhs.y * self,
            z: _rhs.z * self,
        }
    }
}
impl Div<Vec3> for i64 {
    type Output = Vec3;
    fn div(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: _rhs.x / I34F30::from_num(self as i64),
            y: _rhs.y / I34F30::from_num(self as i64),
            z: _rhs.z / I34F30::from_num(self as i64),
        }
    }
}
impl Div<Vec3> for I34F30 {
    type Output = Vec3;
    fn div(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: _rhs.x / self,
            y: _rhs.y / self,
            z: _rhs.z / self,
        }
    }
}

impl Shl<i64> for Vec3 {
    type Output = Vec3;
    fn shl(self, _rhs: i64) -> Vec3 {
        Vec3 {
            x: self.x << _rhs as i64,
            y: self.y << _rhs as i64,
            z: self.z << _rhs as i64,
        }
    }
}
impl Shr<i64> for Vec3 {
    type Output = Vec3;
    fn shr(self, _rhs: i64) -> Vec3 {
        Vec3 {
            x: self.x >> _rhs as i64,
            y: self.y >> _rhs as i64,
            z: self.z >> _rhs as i64,
        }
    }
}

impl Vec3 {
    pub fn length_squared(&self) -> I34F30 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }
    pub fn length(&self) -> I34F30 {
        return self.length_squared().sqrt();
    }

    pub fn unit_vector(&self) -> Vec3 {
        return self.clone() / self.length();
    }

    pub fn dot_prod(&self, other: Vec3) -> I34F30 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }
    pub fn cross_prod(&self, other: Vec3) -> Vec3 {
        return Vec3 {
            x: self.y * other.z - self.x * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        };
    }

    pub const fn new(_x: I34F30, _y: I34F30, _z: I34F30) -> Vec3 {
        return Vec3 {
            x: _x,
            y: _y,
            z: _z,
        };
    }
    pub const fn newi(_x: i64, _y: i64, _z: i64) -> Vec3 {
        return Vec3 {
            x: I34F30::const_from_int(_x as i64),
            y: I34F30::const_from_int(_y as i64),
            z: I34F30::const_from_int(_z as i64),
        };
    }

    pub fn rand(t: &Timer) -> Vec3 {
        return Vec3 {
            x: rand_double(t),
            y: rand_double(t),
            z: rand_double(t),
        };
    }
    pub fn rand_range(t: &Timer, min: I34F30, max: I34F30) -> Vec3 {
        return Vec3 {
            x: rand_double_range(t, min, max),
            y: rand_double_range(t, min, max),
            z: rand_double_range(t, min, max),
        };
    }

    pub fn near_zero(&self) -> bool {
        let s = I34F30::from_num(1e-8);
        return self.x < s && self.y < s && self.z < s;
    }

    pub fn idx(&self, i: i8) -> I34F30 {
        let i = i % 3;
        if i == 0 {return self.x;}
        else if i == 1 {return self.y;}
        else {return self.z;}
    }
}
