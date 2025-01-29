use bytemuck::cast;

use super::Fixed32;
use crate::math::types::FRACTIONAL;

impl Fixed32 {
    
    pub fn next_rand_full_range(&mut self) -> Fixed32 {
        self.inner = cast::<u32, i32>(
            cast::<i32, u32>(self.inner)
                .wrapping_mul(0x01010101)
                .wrapping_add(0x31415927),
        );
        Self { inner: self.inner }
    }
    
    pub fn next_rand_frac(&mut self) -> Fixed32 {
        self.inner = cast::<u32, i32>(
            cast::<i32, u32>(self.inner)
                .wrapping_mul(0x01010101)
                .wrapping_add(0x31415927),
        );
        Self {
            inner: (self.inner >> const { 32 - FRACTIONAL }).wrapping_add((self.inner & 0b10) >> 1),
        }
    }
    
    pub fn next_rand_minmax(&mut self, min: Fixed32, max: Fixed32) -> Fixed32 {
        self.inner = cast::<u32, i32>(
            cast::<i32, u32>(self.inner)
                .wrapping_mul(0x01010101)
                .wrapping_add(0x31415927),
        );
        (Self {
                inner: (self.inner >> const { 32 - FRACTIONAL }).wrapping_add((self.inner & 0b10) >> 1)
            } * Self {
                inner: max.inner.wrapping_sub(min.inner),
            })
    }
}
