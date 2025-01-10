use super::Fixed32;
use agb::println;
use micromath::F32Ext;

impl<const FRACTIONAL: usize> Fixed32<FRACTIONAL> {
    #[inline]
    pub fn recip(&self) -> Self {
        if self.inner <= -(1<<14) || self.inner >= (1<<14) {
            return Self::zero();
        }

        let offset = ((1 << 14) + self.inner as isize) as usize;
        return Self::gen_reciprocal_table()[offset];
    }


    pub const fn gen_reciprocal_table() -> [Self; 2<<14] {
        let mut table = [Self::zero(); 2<<14];
        let mut i = 0;
        while i < 2<<14 {
            table[i] = Self::from_f32(
                ((1 << FRACTIONAL) as f32 / (i as f32 - (1<<14) as f32))
            );
            i += 1;
        }
        table
    }
}
