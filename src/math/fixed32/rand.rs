use super::Fixed32;
use crate::math::types::FRACTIONAL;

impl Fixed32 {
    #[link_section = ".iwram"]
    pub fn next_rand_full_range(&mut self) -> Fixed32 {
        self.inner ^= self.inner << 13;
        self.inner ^= self.inner >> 17;
        self.inner ^= self.inner << 5;
        Self {
            inner: self.inner
        }
    }
    #[link_section = ".iwram"]
    pub fn next_rand_frac(&mut self) -> Fixed32 {
        self.inner ^= self.inner << 13;
        self.inner ^= self.inner >> 17;
        self.inner ^= self.inner << 5;
        Self {
            inner: (self.inner >> const { 32 - FRACTIONAL }).wrapping_add((self.inner & 0b10) >> 1)
        }
    }
}