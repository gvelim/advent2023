mod dish;

use dish::{ReflectorDish, Direction};
use std::collections::HashMap;

fn main() {
    let inp = std::fs::read_to_string("src/bin/day14/input.txt").expect("Ops!");
    let dish = &mut inp.parse::<ReflectorDish>().unwrap_or_default();

    let t = std::time::Instant::now();
    println!("Part 1: Total load = {:?} - {:?}",dish.tilt(Direction::North),t.elapsed());

    let mut map = HashMap::<Vec<u8>,usize>::new();

    let t = std::time::Instant::now();
    let cost = (1..1000)
        .map(|round| (
            round,
            dish.spin_cycle(),
            map.insert(dish.layout.clone(), round)
        ))
        .skip_while(|(round, _, seen)|
            seen.map(|last|
                (1000000000 - last) % (round - last) != 0
            ).unwrap_or(true)
        )
        .map(|(_,cost,_)| cost)
        .next();

    println!("Part 2: Total load = {:?} - {:?}", cost, t.elapsed()
    );
}

