use super::fixed32::Fixed32;

pub const FRACTIONAL: usize = 16;
pub type FixFlt = Fixed32; // Fixed float, just a fixed point number, used to have a single type through the program if i need to adjust it.

#[derive(Clone, Copy)]
pub struct FixFltOnce {
    pub inner: FixFlt,
    initialized: bool
}

impl PartialEq for FixFltOnce {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}
impl PartialEq<FixFlt> for FixFltOnce {
    fn eq(&self, other: &FixFlt) -> bool {
        self.inner == *other
    }
}

impl Into<FixFlt> for FixFltOnce {
    fn into(self) -> FixFlt {
        self.inner
    }
}

impl FixFltOnce {
    
    pub fn init_and_get<F>(&mut self, init: F) -> FixFlt where F: Fn() -> FixFlt {
        if !self.initialized {
            self.inner = init();
            self.initialized = true;
        }
        self.inner
    }

    
    pub const fn new() -> Self {
        Self {
            inner: FixFlt::from_f32(0.0),
            initialized: false
        }
    }
}
