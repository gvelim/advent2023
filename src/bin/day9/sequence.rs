use crate::predictor::*;
use std::str::FromStr;

pub(crate) type Number = i32;

#[derive(Debug, PartialEq)]
pub(crate) struct Sequence {
    pub(crate) history: Vec<Number>
}

impl Sequence {
    pub(crate) fn get_fwd_predictor(&self) -> FwdPredictor {
        FwdPredictor { seq: self.history.clone() }
    }
    pub(crate) fn get_bkwd_predictor(&self) -> BkwdPredictor {
        let mut seq = self.history.clone();
        seq.reverse();
        BkwdPredictor { seq }
    }
}

impl FromStr for Sequence {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Sequence {
            history: s.split_ascii_whitespace()
                .map(|s| s.parse::<Number>().expect("Ops!"))
                .collect::<Vec<_>>()
        })
    }
}
