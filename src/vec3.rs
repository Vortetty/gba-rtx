use core::{ops::{Add, Mul, Div, Sub, Neg}, i16::MIN};

use agb::timer::Timer;
use agb_fixnum::{Num, num};

use crate::rand::{rand_double, rand_double_range};

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: Num<i32, 16>,
    pub y: Num<i32, 16>,
    pub z: Num<i32, 16>,
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
impl Mul<i32> for Vec3 {
    type Output = Self;
    fn mul(self, _rhs: i32) -> Self {
        Self {
            x: self.x * Num::new(_rhs),
            y: self.y * Num::new(_rhs),
            z: self.z * Num::new(_rhs),
        }
    }
}
impl Mul<Num<i32, 16>> for Vec3 {
    type Output = Self;
    fn mul(self, _rhs: Num<i32, 16>) -> Self {
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
impl Div<i32> for Vec3 {
    type Output = Self;
    fn div(self, _rhs: i32) -> Self {
        Self {
            x: self.x / Num::new(_rhs),
            y: self.y / Num::new(_rhs),
            z: self.z / Num::new(_rhs),
        }
    }
}
impl Div<Num<i32, 16>> for Vec3 {
    type Output = Self;
    fn div(self, _rhs: Num<i32, 16>) -> Self {
        Self {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
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
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<Vec3> for i32 {
    type Output = Vec3;
    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: _rhs.x * Num::new(self),
            y: _rhs.y * Num::new(self),
            z: _rhs.z * Num::new(self),
        }
    }
}
impl Mul<Vec3> for Num<i32, 16> {
    type Output = Vec3;
    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: _rhs.x * self,
            y: _rhs.y * self,
            z: _rhs.z * self,
        }
    }
}
impl Div<Vec3> for i32 {
    type Output = Vec3;
    fn div(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: _rhs.x / Num::new(self),
            y: _rhs.y / Num::new(self),
            z: _rhs.z / Num::new(self),
        }
    }
}
impl Div<Vec3> for Num<i32, 16> {
    type Output = Vec3;
    fn div(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: _rhs.x / self,
            y: _rhs.y / self,
            z: _rhs.z / self,
        }
    }
}

impl Vec3 {
    pub fn length_squared(&self) -> Num<i32, 16> {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }
    pub fn length(&self) -> Num<i32, 16> {
        return self.length_squared().sqrt();
    }

    pub fn unit_vector(&self) -> Vec3 {
        return self.clone() / self.length();
    }

    pub fn dot_prod(&self, other: Vec3) -> Num<i32, 16> {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }
    pub fn cross_prod(&self, other: Vec3) -> Vec3 {
        return Vec3 {
            x: self.y * other.z - self.x * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        };
    }

    pub fn new(_x: Num<i32, 16>, _y: Num<i32, 16>, _z: Num<i32, 16>) -> Vec3 {
        return Vec3 {
            x: _x,
            y: _y,
            z: _z,
        };
    }
    pub fn newi(_x: i32, _y: i32, _z: i32) -> Vec3 {
        return Vec3 {
            x: Num::new(_x),
            y: Num::new(_y),
            z: Num::new(_z),
        };
    }

    pub fn rand(t: &Timer) -> Vec3 {
        return Vec3 {
            x: rand_double(t),
            y: rand_double(t),
            z: rand_double(t),
        };
    }
    pub fn rand_range(t: &Timer, min: Num<i32, 16>, max: Num<i32, 16>) -> Vec3 {
        return Vec3 {
            x: rand_double_range(t, min, max),
            y: rand_double_range(t, min, max),
            z: rand_double_range(t, min, max),
        };
    }

    pub fn near_zero(&self) -> bool {
        let s = Num::from_raw(1);
        return self.x < s && self.y < s && self.z < s;
    }

    pub fn idx(&self, i: i8) -> Num<i32, 16> {
        let i = i % 3;
        if i == 0 {return self.x;}
        else if i == 1 {return self.y;}
        else {return self.z;}
    }
}
