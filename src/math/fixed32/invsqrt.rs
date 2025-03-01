use super::{sqrt::sqrt_const, Fixed32};
use crate::math::types::FRACTIONAL;

impl Fixed32 {
    pub fn invsqrt(&self) -> Self {
        let mut scale = 0usize;
        let mut x = if self.inner < 0 {
            return Fixed32::zero();
        } else {
            self.inner
        };

        while x >= (INVSQRT_LUT.len()) as i32 {
            x >>= 1;
            scale += 1;
        }

        unsafe {
            (*INVSQRT_LUT.get_unchecked(x as usize)) >> (scale / 2)
        }
    }

    // python3:
    // python -c 'for i in range(16384): print(f"        Fixed32::from_f32({__import__("math").sqrt(i+1)}),")' > SQRT_LUT.txt
}

// Sqrt for range 0-anything
pub(super) const fn invsqrt_const(x: f32) -> f32 {
    let (mut l, mut h) = (0.0, if x > 1.0 { x } else { 1.0 });
    let mut ctr = 0usize;
    const LOOPS: usize = 100;

    loop {
        ctr += 1;
        if ctr > LOOPS {
            break;
        }
        let mid = (l + h) * 0.5;
        if mid * mid > 1.0 / x {
            h = mid;
        } else {
            l = mid;
        }
    }
    (l + h) * 0.5
}

const INVSQRT_LUT_SIZE: usize = 1<<18 + 1;
#[allow(long_running_const_eval)]
static INVSQRT_LUT: [Fixed32; INVSQRT_LUT_SIZE] = get_invsqrt_lut();
const fn get_invsqrt_lut() -> [Fixed32; INVSQRT_LUT_SIZE] {
    let mut out = [Fixed32::zero(); INVSQRT_LUT_SIZE];
    let mut ctr = 0usize;

    loop {
        if ctr >= INVSQRT_LUT_SIZE {
            break;
        } else {
            ctr += 1;
            out[ctr-1] = Fixed32::from_f32(invsqrt_const(Fixed32{inner: ctr as i32}.as_f32()));
        }
    }

    out
}
