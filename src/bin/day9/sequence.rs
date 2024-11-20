use crate::predictor::*;
use std::{str::FromStr, rc::Rc};

pub(crate) type Number = i32;

#[derive(Debug, PartialEq)]
pub(crate) struct Sequence {
    pub(crate) history: Rc<[Number]>
}

impl Sequence {
    pub(crate) fn get_fwd_predictor(&self) -> FwdPredictor {
        FwdPredictor::new(&self.history)
    }
    pub(crate) fn get_bkwd_predictor(&self) -> BkwdPredictor {
        BkwdPredictor::new(&self.history)
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
