use crate::sequence::*;

pub(crate) struct FwdPredictor {
    pub(crate) seq: Vec<Number>
}
impl FwdPredictor {
    fn predict_next(history: &[i32]) -> i32 {
        let reduced = history.windows(2).map(|a| a[1]-a[0]).collect::<std::rc::Rc<[_]>>();
        if reduced.iter().all(|d| d.eq(&0)) {
            return history[0];
        }
        Self::predict_next(&reduced) + history[reduced.len()]
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
    pub(crate) seq: Vec<Number>
}

impl BkwdPredictor {
    fn predict_bwd(history: &[i32]) -> i32 {
        let reduced = history.windows(2).map(|a| a[0]-a[1]).collect::<std::rc::Rc<[_]>>();
        if reduced.iter().all(|d| d.eq(&0)) {
            return history[0];
        }
        history[reduced.len()] - Self::predict_bwd(&reduced)
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
