mod digging_plan;
mod instruction;
mod lagoon;
mod position;

use digging_plan::DigPlan;
use lagoon::{Digger, Lagoon};
use position::Position;

fn main() {
    let plan = std::fs::read_to_string("./src/bin/day18/input.txt")
        .expect("ops")
        .parse::<DigPlan>()
        .expect("Ops");

    let mut lagoon = Lagoon::default();
    let mut digger = Digger::new(Position(0, 0));

    let t = std::time::Instant::now();
    let total = plan
        .iter()
        .map(|ins| digger.dig(&mut lagoon, ins))
        .sum::<usize>();
    let area = lagoon.calculate_area();
    println!("\nPart 1:\n\tLagoon Periphery {}\n\tLagoon area = {}\nTotal: {} - {:?}",
        total, area, total + area,t.elapsed()
    );
    assert_eq!(40714,total + area);

    let mut lagoon = Lagoon::default();
    let mut digger = Digger::new(Position(0, 0));

    let t = std::time::Instant::now();
    let total = plan
        .iter()
        .map(|ins|
            digger.dig(
                &mut lagoon,
                &ins.decode_rgb().expect("ops")
            )
        )
        .sum::<usize>();
    let area = lagoon.calculate_area();
    println!("\nPart 2:\n\tLagoon Periphery {}\n\tLagoon area = {}\nTotal: {} - {:?}",
        total, area, total + area,t.elapsed()
    );
    assert_eq!(129849166997110,total + area);
}
