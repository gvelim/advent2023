mod sequence;
mod iterator;

use crate::sequence::{Sequence,Number};

fn main() {
    let input = std::fs::read_to_string("src/bin/day9/input.txt").expect("Ops!");
    let mut seqs = input
        .lines()
        .map(|line| line
            .parse::<Sequence>()
            .unwrap_or_else(|e| panic!("Ops! {} -> {:?}",e, line))
        )
        .collect::<Vec<_>>();

    let t = std::time::Instant::now();
    let sum = seqs
        .iter_mut()
        .map(|seq| seq.iter_forward().next().unwrap() )
        .sum::<Number>();

    println!("Part 1 - Sum of forward predictions: {sum} - {:?}", t.elapsed());

    let t = std::time::Instant::now();
    let sum = seqs
        .iter_mut()
        .map(|seq| seq.iter_backward().next().unwrap() )
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
