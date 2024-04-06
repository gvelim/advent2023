use std::ops::Range;
use crate::{
    block::{Heat, Step},
    citymap::CityMap,
    direction::Direction as D
};

mod citymap;
mod direction;
mod crucible;
mod block;
mod path;

fn main() {
    let input = std::fs::read_to_string("src/bin/day17/input.txt").expect("File Not Found!");
    let map = input.parse::<CityMap>().expect("ops");

    let total_heat_loss = |rng: Range<Step>| -> Option<Heat> {
        map.get_crucible(0, D::Right)
            .find_path_to(map.len()-1, rng)
            .map(|path| path.total_heat_loss() )
    };

    let t = std::time::Instant::now();
    let loss = total_heat_loss(0..3);
    println!("Part 1: {:?} = {:?}", loss, t.elapsed());
    assert_eq!(loss, Some(1008), "Part 1 result doesn't match");

    let t = std::time::Instant::now();
    let loss = total_heat_loss(4..10);
    println!("Part 2: {:?} = {:?}", loss, t.elapsed());
    assert_eq!(loss, Some(1210), "Part 2 result doesn't match");
}
