mod sequence;
mod predictor;

use crate::sequence::{Sequence,Number};

fn main() {
    let input = std::fs::read_to_string("src/bin/day9/input.txt").expect("Ops!");
    let mut seqs = input.lines()
        .map(|line| line.parse::<Sequence>().expect("Ops!"))
        .collect::<Vec<_>>();

    let t = std::time::Instant::now();
    let sum = seqs
        .iter_mut()
        .map(|seq| seq.get_fwd_predictor().next().unwrap() )
        .sum::<Number>();

    println!("Part 1 - Sum of forward predictions: {sum} - {:?}", t.elapsed());

    let t = std::time::Instant::now();
    let sum = seqs
        .iter_mut()
        .map(|seq| seq.get_bkwd_predictor().next().unwrap() )
        .sum::<Number>();

    println!("Part 2 - Sum of backward predictions: {sum} - {:?}", t.elapsed());

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
                let a = seq.get_bkwd_predictor().next().unwrap();
                (seq, a)
            })
            .map(|(s,r)| {
                println!("{:?} -> {:?}",s,r);
                r
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
                let a = seq.get_fwd_predictor().next();
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
