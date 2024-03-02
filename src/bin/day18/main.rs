mod instruction;
mod digging_plan;
mod position;
mod lagoon;

use digging_plan::DigPlan;
use lagoon::{Digger, Lagoon};
use position::Position;


fn main() {
    let plan = std::fs::read_to_string("./src/bin/day18/input.txt")
        .expect("ops")
        .parse::<DigPlan>()
        .expect("Ops");

    let mut lagoon = Lagoon::default();
    let mut digger = Digger::new(Position(0, 0), 1);

    let total = plan
        .iter()
        .map(|ins| digger.dig(&mut lagoon, ins))
        .sum::<usize>();

    println!("Tranch = {total}");
    println!("{:?}",lagoon);
}
