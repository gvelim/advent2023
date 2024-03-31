#![feature(iter_collect_into)]

mod workflow;
mod rule;
mod part;
mod system;

use std::rc::Rc;
use crate::{system::SortingSystem, part::{Part,Unit}, rule::Action};

fn main() {
    let (parts, system) = parse_puzzle_data("src/bin/day19/input.txt");

    let t = std::time::Instant::now();
    let sum = parts.iter()
        .filter(|&&part|
            system.process_part(part, "in") == Some(Action::Accept)
        )
        .map(|part| part.sum())
        .sum::<Unit>();

    println!("Part 1: Sum of approved parts: {sum} ({:?})", t.elapsed());
    assert_eq!(sum,287_054);

    let t = std::time::Instant::now();
    let sum = system.total_combinations("in", &[1..4001, 1..4001, 1..4001, 1..4001]);
    println!("Part 2: Total combinations: {sum}, ({:?})", t.elapsed());
    assert_eq!(sum,131_619_440_296_497);
}

fn parse_puzzle_data(file: &str) -> (Rc<[Part]>, SortingSystem) {
    let inp = std::fs::read_to_string(file)
        .expect("cannot load data file");
    let mut split = inp.split("\n\n");
    let wfs = split
        .next()
        .unwrap()
        .parse::<SortingSystem>()
        .expect("Failed to parse workflows");

    let parts = split
        .next()
        .unwrap()
        .lines()
        .map(|line| line.parse::<Part>().expect("msg") )
        .collect::<Rc<[Part]>>();

    (parts,wfs)
}
