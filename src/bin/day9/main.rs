use std::str::FromStr;

fn main() {

}

type Number = i32;

#[derive(Debug, PartialEq)]
struct Sequence {
    history: Vec<Number>
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

#[cfg(test)]
mod test {
    use super::*;
    static INPUT: &str = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";

    #[test]
    fn test_sequence_iter() {
        let mut seqs = INPUT.lines()
            .map(|line| line.parse::<Sequence>().expect("Ops!"))
            .collect::<Vec<_>>();

        let sum = seqs.iter_mut()
            .map(|seq| {
                let a = seq.next();
                (seq, a)
            })
            .map(|(s,r)| {
                println!("{:?} -> {:?}",s,r);
                r.unwrap()
            })
            .sum::<Number>();

        assert_eq!(sum,114);
    }
    #[test]
    fn test_parse() {
        let seq = INPUT.lines()
            .map(|line| line.parse::<Sequence>().expect("Ops!"))
            .collect::<Vec<_>>();

        seq.iter().for_each(|s| println!("{:?}",s) );
        assert_eq!(
            seq,
            [
                Sequence { history: vec![0, 3, 6, 9, 12, 15] },
                Sequence { history: vec![1, 3, 6, 10, 15, 21] },
                Sequence { history: vec![10, 13, 16, 21, 30, 45] }
            ]
        )

    }

}
