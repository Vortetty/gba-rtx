use core::mem;

use agb::{timer::Timer};
use agb_fixnum::Num;
use fixed::types::I20F12;

const FNV_PRIME: u32 = 16777619;
const FNV_OFFSET_BASIS: u32 = 2166136261;

#[allow(arithmetic_overflow)]
pub fn fnv1a_hash_u16(i: u16) -> u32 {
    let mut hash = FNV_OFFSET_BASIS;

    for shift in 0..mem::size_of_val(&i) {
        hash = hash ^ (((i >> ((shift * 8) as u16)) & 0b11111111) as u32);
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    return hash;
}

pub fn rand_double(t: &Timer) -> I20F12 {
    return I20F12::from_num(rand_u32(t) as i32) / I20F12::MAX;
}

pub fn rand_double_range(t: &Timer, min: I20F12, max: I20F12) -> I20F12 {
    return min + (max-min) * rand_double(t);
}

pub fn rand_u32(t: &Timer) -> u32 {
    return fnv1a_hash_u16(t.value());
}