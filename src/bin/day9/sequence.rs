use std::str::FromStr;

pub(crate) type Number = i32;

#[derive(Debug, PartialEq)]
pub(crate) struct Sequence {
    pub(crate) history: Vec<Number>
}

impl Sequence {
    fn predict_next(history: &[i32]) -> i32 {
        let reduced = history.windows(2).map(|a| a[1]-a[0]).collect::<Vec<_>>();
        if reduced.iter().all(|d| d.eq(&0)) {
            return history[0];
        } else {
            Self::predict_next(&reduced) + history[reduced.len()]
        }
    }

    pub(crate) fn predict_bwd(history: &[i32]) -> i32 {
        let reduced = history.windows(2).map(|a| a[0]-a[1]).collect::<Vec<_>>();
        if reduced.iter().all(|d| d.eq(&0)) {
            return history[0];
        } else {
            history[reduced.len()] - Self::predict_bwd(&reduced)
        }
    }
}

impl Iterator for Sequence {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let p = Sequence::predict_next(&self.history);
        self.history.push(p);
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
