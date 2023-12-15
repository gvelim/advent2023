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

    fn predict_next(history: &[i32]) -> i32 {
        let reduced = history.windows(2).map(|a| a[1]-a[0]).collect::<Vec<_>>();
        if reduced.iter().all(|d| d.eq(&0)) {
            return history[0];
        }
        Self::predict_next(&reduced) + history[reduced.len()]
    }

    fn predict_bwd(history: &[i32]) -> i32 {
        let reduced = history.windows(2).map(|a| a[0]-a[1]).collect::<Vec<_>>();
        if reduced.iter().all(|d| d.eq(&0)) {
            return history[0];
        }
        history[reduced.len()] - Self::predict_bwd(&reduced)
    }
}
pub(crate) struct FwdPredictor {
    seq: Vec<Number>
}
impl Iterator for FwdPredictor {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let p = Sequence::predict_next(&self.seq);
        self.seq.push(p);
        Some(p)
    }
}

pub(crate) struct BkwdPredictor {
    seq: Vec<Number>
}
impl Iterator for BkwdPredictor {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let p = Sequence::predict_bwd(&self.seq);
        self.seq.push(p);
        Some(p)
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
