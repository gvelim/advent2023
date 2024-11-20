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

pub(crate) struct FwdIterator {
    seq: Vec<Number>
}

impl FwdIterator {
    pub(crate) fn new(vec: &[Number]) -> FwdIterator {
        FwdIterator { seq: vec.to_vec() }
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

impl Iterator for FwdIterator {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let p = FwdIterator::predict_next(&self.seq);
        self.seq.push(p);
        Some(p)
    }
}

pub(crate) struct BkwIterator {
    seq: Vec<Number>
}

impl BkwIterator {
    pub fn new(vec: &[Number]) -> BkwIterator {
        let mut seq = vec.to_vec();
        seq.reverse();
        BkwIterator { seq }
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
impl Iterator for BkwIterator {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let p = BkwIterator::predict_bwd(&self.seq);
        self.seq.push(p);
        Some(p)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    static INPUT: &str = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";

    #[test]
    fn test_predict_bwd() {
        let mut seqs = INPUT.lines()
            .map(|line| line.parse::<Sequence>().expect("Ops!"))
            .collect::<Vec<_>>();

        let sum = seqs.iter_mut()
            .map(|seq| {
                let a = seq.iter_backward().next();
                (seq, a)
            })
            .map(|(s,r)| {
                println!("{:?} -> {:?}",s,r);
                r.unwrap()
            })
            .sum::<Number>();

        assert_eq!(sum,2);
    }

    #[test]
    fn test_predict_fwd() {
        let mut seqs = INPUT.lines()
            .map(|line| line.parse::<Sequence>().expect("Ops!"))
            .collect::<Vec<_>>();

        let sum = seqs.iter_mut()
            .map(|seq| {
                let a = seq.iter_forward().next();
                (seq, a)
            })
            .map(|(s,r)| {
                println!("{:?} -> {:?}",s,r);
                r.unwrap()
            })
            .sum::<Number>();

        assert_eq!(sum,114);
    }
}
