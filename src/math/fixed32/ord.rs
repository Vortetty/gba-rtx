use super::Fixed32;


impl<const FRACTIONAL: usize> PartialOrd for Fixed32<FRACTIONAL> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<const FRACTIONAL: usize> Ord for Fixed32<FRACTIONAL> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}