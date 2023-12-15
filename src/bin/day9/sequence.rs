use std::str::FromStr;

pub(crate) type Number = i32;

#[derive(Debug, PartialEq)]
pub(crate) struct Sequence {
    pub(crate) history: Vec<Number>
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

impl Sequence {
    fn predict(history: &[i32]) -> i32 {
        // println!("H: {:?}", history);
        let reduced = history.windows(2).map(|a| a[1]-a[0]).collect::<Vec<_>>();
        if reduced.iter().all(|d| d.eq(&0)) {
            return history[0];
        } else {
            let a = Self::predict(&reduced) + history[reduced.len()];
            // println!("{:?}",(&history, &reduced));
            a
        }
    }

}
impl Iterator for Sequence {
    type Item = Number;
    fn next(&mut self) -> Option<Self::Item> {
        let p = Sequence::predict(&self.history);
        self.history.push(p);
        Some(p)
    }
}
