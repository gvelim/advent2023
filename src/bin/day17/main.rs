use crate::citymap::CityMap;

mod citymap;
mod direction;
mod crucible;

fn main() {
    let input = std::fs::read_to_string("src/bin/day17/input.txt").expect("File Not Found!");
    let map = input.parse::<CityMap>().expect("ops");

    let mut c = map.get_crucible(0, crate::direction::Direction::Right);
    println!("{:?}",c.heat_to_target_block_a(map.len()-1));

}






