use std::cmp::Ordering;
use crate::direction::Direction;

pub(crate) type Heat = u16;
pub(crate) type Position = usize;
pub(crate) type Step = usize;

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Copy, Clone)]
pub(crate) struct CityBlock(pub Position, pub Direction, pub Step);

#[derive(Debug, Eq)]
pub(crate) struct QueuedCityBlock(pub Heat, pub CityBlock);

impl PartialEq<Self> for QueuedCityBlock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd<Self> for QueuedCityBlock {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueuedCityBlock {
    fn cmp(&self, other: &Self) -> Ordering {
        // reverse comparison so BinaryHeap orders by min value first
        other.0.cmp(&self.0)
            // if Cityblocks have heat_loss sum, prefer the one with the longer step run
            .then_with(|| self.1.2.cmp(&other.1.2))
    }
}
