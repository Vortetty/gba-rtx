use super::Fixed32;


impl<const FRACTIONAL: usize> Fixed32<FRACTIONAL> {
    #[inline]
    pub fn sqrt(&self) -> Self {
        let mut x = 1 << (FRACTIONAL);
        for _ in 0..4 {
            x = ((x * 2) >> FRACTIONAL) - ((self.inner * x) >> FRACTIONAL);
        }
        Self {
            inner: x
        }
    }
}