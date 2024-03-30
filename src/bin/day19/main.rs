#![feature(iter_collect_into)]

mod workflow;
mod rule;
mod part;
mod system;

use std::rc::Rc;
use crate::{system::SortingSystem, part::{Part,Unit}, rule::Action};

fn main() {
    let (parts, system) = parse_puzzle_data("src/bin/day19/input.txt");

    let sum = parts.iter()
        .filter(|&&part|
            system.process_part(part, "in") == Some(Action::Accept)
        )
        .map(|part| part.sum())
        .sum::<Unit>();

    println!("Part 1: Sum of approved parts: {sum}");
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
