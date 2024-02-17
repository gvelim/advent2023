use crate::citymap::CityMap;

mod citymap;
mod direction;
mod crucible;

fn main() {
    let input = std::fs::read_to_string("src/bin/day17/input.txt").expect("File Not Found!");
    let map = input.parse::<CityMap>().expect("ops");

    let mut c = map.get_crucible(0, crate::direction::Direction::Right);
    let t = std::time::Instant::now();
    println!("Part 1: {:?} = {:?}",c.heat_to_target_block(map.len()-1, 1..3), t.elapsed());

    let mut c = map.get_crucible(0, crate::direction::Direction::Right);
    let t = std::time::Instant::now();
    println!("Part 2: {:?} = {:?}",c.heat_to_target_block(map.len()-1, 4..10), t.elapsed());

}