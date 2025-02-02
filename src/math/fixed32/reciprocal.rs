use super::Fixed32;
use crate::math::types::FRACTIONAL;

impl Fixed32 {
    pub fn recip(&self) -> Self {
        let mut scale = 0usize;
        let mut neg = false;
        let mut x = if self.inner <= 0 {
            neg = true;
            -self.inner
        } else {
            self.inner
        };

        while x >= (RECIP_LUT.len()) as i32 {
            x = x >> 1;
            scale += 1;
        }


        if neg {
            -((RECIP_LUT[x as usize]) >> scale)
        } else {
            (RECIP_LUT[x as usize]) >> scale
        }
    }

    // python3:
    // python -c "for i in range((2**12)*4): print(f'        (Self::from_f32({1.0/float((i+1)/4)}).inner >> 3) as i16,')" > RECIP_LUT.txt
}

const RECIP_LUT_SIZE: usize = (1<<16);
static RECIP_LUT: [Fixed32; RECIP_LUT_SIZE] = get_recip_lut();
const fn get_recip_lut() -> [Fixed32; RECIP_LUT_SIZE] {
    let mut out = [Fixed32::zero(); RECIP_LUT_SIZE];
    let mut ctr = 0usize;

    loop {
        if ctr >= RECIP_LUT_SIZE {
            break;
        } else {
            ctr += 1;
            out[ctr-1] = Fixed32::from_f32((1.0 / Fixed32{inner: ctr as i32}.as_f32()));
        }
    }

    out
}
