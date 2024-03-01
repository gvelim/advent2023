use crate::instruction::Direction;
use std::{
    cmp::Ordering,
    fmt::{Debug, Formatter},
};

pub(crate) type Unit = i16;

#[derive(Eq, PartialEq, Clone, Copy)]
pub(crate) struct Position(pub Unit, pub Unit);

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

impl PartialOrd<Self> for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1).then_with(|| self.0.cmp(&other.0))
    }
}

impl Position {
    pub(crate) fn next_mut(&mut self, dir: Direction) -> &mut Self {
        match dir {
            Direction::U => self.1 -= 1,
            Direction::R => self.0 += 1,
            Direction::D => self.1 += 1,
            Direction::L => self.0 -= 1,
        };
        self
    }
    pub(crate) fn next(&self, dir: Direction) -> Self {
        let mut out = *self;
        match dir {
            Direction::U => out.1 -= 1,
            Direction::R => out.0 += 1,
            Direction::D => out.1 += 1,
            Direction::L => out.0 -= 1,
        };
        out
    }
}
