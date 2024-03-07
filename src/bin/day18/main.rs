mod digging_plan;
mod instruction;
mod lagoon;
mod position;

use digging_plan::DigPlan;
use lagoon::{Digger, Lagoon};
use position::Position;
use crate::instruction::Instruction;

fn main() {
    type FnEncode = fn(&Instruction) -> Instruction;

    let plan = match std::fs::read_to_string("./src/bin/day18/input.txt")
        .expect("Cannot Find File")
        .parse::<DigPlan>() {
            Ok(plan) => plan,
            Err(e) => panic!("{}",e),
        };

    let dig_lagoon =
        |enc: FnEncode| -> usize {
            let mut lagoon = Lagoon::default();
            let mut digger = Digger::new(Position(0, 0));

            let t = std::time::Instant::now();
            let total = plan
                .iter()
                .map(|ins| digger.dig(&mut lagoon, &enc(ins)))
                .sum::<usize>();

            let area = lagoon.calculate_area();
            println!("\nPart 1:\n\tLagoon Periphery {}\n\tLagoon area = {}\nTotal: {} - {:?}",
                total, area, total + area,t.elapsed()
            );

            total + area
    };

    let pass_through: FnEncode = |i| { i.clone() };
    let encode_rgb: FnEncode =  |i| { i.decode_rgb() };

    assert_eq!(40714, dig_lagoon(pass_through));
    assert_eq!(129849166997110, dig_lagoon(encode_rgb));
}
