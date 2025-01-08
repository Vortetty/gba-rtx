use fixed::types::I14F18;

pub type FixFlt = I14F18; // Fixed float, just a fixed point number, used to have a single type through the program if i need to adjust it. 

#[derive(Clone, Copy, Eq)]
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
    #[inline(always)]
    pub fn init_and_get<F>(&mut self, init: F) -> FixFlt where F: Fn() -> FixFlt {
        if !self.initialized {
            self.inner = init();
            self.initialized = true;
        }
        self.inner
    }

    #[inline(always)]
    pub fn new() -> Self {
        Self {
            inner: FixFlt::lit("0"),
            initialized: false
        }
    }
}