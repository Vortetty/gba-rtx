use super::Fixed32;

impl PartialOrd for Fixed32 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl Ord for Fixed32 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}