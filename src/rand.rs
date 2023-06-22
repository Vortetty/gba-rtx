use core::{mem, cell::RefCell};

use agb::{timer::Timer, rng::RandomNumberGenerator, sync::Mutex};
use agb_fixnum::Num;

pub static rng: Mutex<RefCell<RandomNumberGenerator>> = Mutex::new(RefCell::new(RandomNumberGenerator::new()));

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

struct RandomNumberGeneratorModifiable {
    pub state: [u32; 4],
}

pub fn seed_rng_from_timer(t: &Timer) {
    unsafe {
        let rngptr: *mut RandomNumberGenerator = rng.lock().get_mut();
        (*core::mem::transmute::<*const (), *mut RandomNumberGeneratorModifiable>(rngptr as *const ())).state = [
            fnv1a_hash_u16(t.value()),
            fnv1a_hash_u16(t.value()),
            fnv1a_hash_u16(t.value()),
            fnv1a_hash_u16(t.value())
        ];
    }
}

pub fn rand_double() -> Num<i32, 16> {
    return Num::new(rng.lock().get_mut().gen());
}

pub fn rand_double_range(min: i32, max: i32) -> Num<i32, 16> {
    return Num::new(min) + Num::new(max-min)*rand_double();
}

pub fn rand_u32() -> u32 {
    return unsafe { mem::transmute(rng.lock().get_mut().gen()) };
}