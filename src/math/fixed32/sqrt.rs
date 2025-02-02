use super::Fixed32;
use crate::math::types::FRACTIONAL;

impl Fixed32 {
    pub fn sqrt(&self) -> Self {
        let mut scale = 0usize;
        let mut x = if self.inner < 0 {
            return Fixed32::zero();
        } else {
            self.inner
        };

        while x >= (SQRT_LUT.len()) as i32 {
            x = x >> 2;
            scale += 1;
        }

        unsafe {
            *SQRT_LUT.get_unchecked(x as usize) << scale >> const { FRACTIONAL / 2 }
        }
    }
    pub fn sqrt01(&self) -> Self {
        unsafe {
            *SQRT01_LUT.get_unchecked((self.inner >> 7) as usize)
        }
    }

    // python3:
    // python -c 'for i in range(16384): print(f"        Fixed32::from_f32({__import__("math").sqrt(i+1)}),")' > SQRT_LUT.txt
}

// Sqrt for range 0-anything
pub(super) const fn sqrt_const(x: f32)  -> f32 {
    let (mut l, mut h) = (0.0, x);
    let mut ctr = 0usize;
    let loops = 100;
    loop {
        ctr += 1;
        if (ctr > loops) { break; }
        let rt = (l + h) * 0.5;
        if rt * rt > x {
            h = rt;
        } else {
            l = rt;
        }
    }
    return (l + h) * 0.5;
}

const SQRT_LUT_SIZE: usize = 1<<16 + 1;
#[allow(long_running_const_eval)]
static SQRT_LUT: [Fixed32; SQRT_LUT_SIZE] = get_sqrt_lut();
const fn get_sqrt_lut() -> [Fixed32; SQRT_LUT_SIZE] {
    let mut out = [Fixed32::zero(); SQRT_LUT_SIZE];
    let mut ctr = 0usize;

    loop {
        if ctr >= SQRT_LUT_SIZE-1 {
            break;
        } else {
            ctr += 1;
            out[ctr] = Fixed32::from_f32(sqrt_const(ctr as f32));
        }
    }

    out
}

// Sqrt for range 0-1
const fn sqrt_const01(x: f32)  -> f32 {
    let (mut l, mut h) = (0.0, x.max(1.0));
    let mut ctr = 0usize;
    let loops = 32;
    loop {
        ctr += 1;
        if (ctr > loops) { break; }
        let rt = (l + h) * 0.5;
        if rt * rt > x {
            h = rt;
        } else {
            l = rt;
        }
    }
    return (l + h) * 0.5;
}
#[link_section = ".ewram"]
static SQRT01_LUT: [Fixed32; 1<<14] = get_sqrt01_lut();
const fn get_sqrt01_lut() -> [Fixed32; 1<<14] {
    let mut out = [Fixed32::zero(); 1<<14];
    let mut ctr = 0i32;

    loop {
        if ctr >= (1<<14)-1 {
            break;
        } else {
            ctr += 1;
            out[ctr as usize] = Fixed32::from_f32(sqrt_const01(Fixed32 {inner: ctr<<7}.as_f32()));
        }
    }

    out
}
