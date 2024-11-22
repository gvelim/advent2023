
mod engine;
mod parts;

use std::time;
use std::time::*;
use crate::engine::*;

fn main() {
    let input = std::fs::read_to_string("src/bin/day3/input.txt").expect("Ops!");
    let es = input.parse::<EngineSchematic>().expect("Ops!");

    let t = Instant::now();
    let sum = es
        .part_numbers()
        .map(|pn| pn.number)
        .sum::<u32>();

    println!("Par 1 - Sum: {sum} - {:?}", t.elapsed());

    let t = time::Instant::now();
    let sum = es
        .get_gears_part_numbers('*')
        .map(|d| d.iter().map(|d| d.number).product::<u32>())
        .sum::<u32>();

    println!("Par 1 - Sum: {sum} - {:?}", t.elapsed());
}
