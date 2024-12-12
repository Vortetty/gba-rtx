use core::mem;

use agb::{timer::Timer, println};


const FNV_PRIME: u32 = 16777619;
const FNV_OFFSET_BASIS: u32 = 2166136261;

#[allow(arithmetic_overflow)]
pub fn fnv1a_hash_u32(i: u32) -> u32 {
    let mut hash = FNV_OFFSET_BASIS;

    for shift in 0..mem::size_of_val(&i) {
        hash = hash ^ (((i >> ((shift * 8) as u32)) & 0b11111111) as u32);
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    return hash;
}

pub fn rand_double(t: &Timer) -> f32 {
    let r = rand_u32(t);
    println!("{}", r);
    return (r as f32) / u32::MAX as f32;
}

pub fn rand_double_range(t: &Timer, min: f32, max: f32) -> f32 {
    return min + (max-min) * rand_double(t);
}

static mut STATE: u32 = 0;

pub fn rand_u32(t: &Timer) -> u32 {
    unsafe {
        STATE = fnv1a_hash_u32(t.value() as u32 ^ STATE);
        return STATE;
    }
}