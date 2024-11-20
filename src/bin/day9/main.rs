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
