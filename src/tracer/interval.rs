use crate::math::types::FixFlt;

#[derive(Clone, Copy)]
pub struct Interval {
    pub min: FixFlt,
    pub max: FixFlt
}

impl Interval {
    pub const fn empty() -> Self {
        Self {
            min: FixFlt::max_val(),
            max: FixFlt::min_val()
        }
    }
    pub const fn universe() -> Self {
        Self {
            min: FixFlt::min_val(),
            max: FixFlt::max_val()
        }
    }
    pub const fn new(min: FixFlt, max: FixFlt) -> Self {
        Self {
            min,
            max
        }
    }

    pub fn size(self) -> FixFlt {
        self.max-self.min
    }

    pub fn contains(self, rhs: FixFlt) -> bool {
        self.min <= rhs && rhs <= self.max
    }

    pub fn surrounds(self, rhs: FixFlt) -> bool {
        self.min < rhs && rhs < self.max
    }

    pub fn clamp(self, rhs: FixFlt) -> FixFlt {
        if self.max < rhs { self.max }
        else if self.min > rhs { self.min }
        else { rhs }
    }
}