use crate::iterator::*;
use std::{str::FromStr, rc::Rc};

pub(crate) type Number = i32;

#[derive(Debug, PartialEq)]
pub(crate) struct Sequence {
    pub(crate) history: Rc<[Number]>
}

impl Sequence {
    pub(crate) fn iter_forward(&self) -> FwdIterator {
        FwdIterator::new(&self.history)
    }
    pub(crate) fn iter_backward(&self) -> BkwIterator {
        BkwIterator::new(&self.history)
    }
}

impl FromStr for Sequence {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Sequence {
            history: s.split_ascii_whitespace()
                .map(|s| s.parse::<Number>().expect("Ops!"))
                .collect::<Rc<[_]>>()
        })
    }
}
