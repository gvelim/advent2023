mod dish;

use dish::{ReflectorDish, Direction};

fn main() {
    let inp = std::fs::read_to_string("src/bin/day14/input.txt").expect("Ops!");
    let dish = &mut inp.parse::<ReflectorDish>().unwrap_or_default();

    let t = std::time::Instant::now();
    println!("Part 1: Total load = {:?} - {:?}",dish.tilt(Direction::North),t.elapsed());

    let t = std::time::Instant::now();
    println!("Part 2: Total load = {:?} - {:?}", dish.spin_cycle_nth(1000000000), t.elapsed()
    );
}

