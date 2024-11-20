use std::rc::Rc;
use crate::sequence::*;

fn reduce_level(
    vec: &[Number],
    pair_calc: fn(&[Number]) -> Number
) -> Rc<[Number]>
{
    vec
        .windows(2)
        .map(pair_calc)
        .collect::<Rc<[Number]>>()
}

pub(crate) struct FwdPredictor {
    seq: Vec<Number>
}

impl FwdPredictor {
    pub(crate) fn new(vec: &[Number]) -> FwdPredictor {
        FwdPredictor { seq: vec.to_vec() }
    }
    fn predict_next(history: &[Number]) -> Number {
        let reduced = reduce_level(history, |a| a[1]-a[0] );
        if reduced.iter().all(|d| 0.eq(d)) {
            history[0]
        } else {
            Self::predict_next(&reduced) + history[reduced.len()]
        }
    }
}

impl Iterator for FwdPredictor {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let p = FwdPredictor::predict_next(&self.seq);
        self.seq.push(p);
        Some(p)
    }
}

pub(crate) struct BkwdPredictor {
    seq: Vec<Number>
}

impl BkwdPredictor {
    pub fn new(vec: &[Number]) -> BkwdPredictor {
        let mut seq = vec.to_vec();
        seq.reverse();
        BkwdPredictor { seq }
    }
    fn predict_bwd(history: &[Number]) -> Number {
        let reduced = reduce_level(history, |a| a[0]-a[1]);
        if reduced.iter().all(|d| 0.eq(d)) {
            history[0]
        } else {
            history[reduced.len()] - Self::predict_bwd(&reduced)
        }
    }
}
impl Iterator for BkwdPredictor {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let p = BkwdPredictor::predict_bwd(&self.seq);
        self.seq.push(p);
        Some(p)
    }
}
