use crate::citymap::CityMap;

mod citymap;
mod direction;
mod crucible;
mod block;
mod path;

fn main() {
    let input = std::fs::read_to_string("src/bin/day17/input.txt").expect("File Not Found!");
    let map = input.parse::<CityMap>().expect("ops");

    let mut c = map.get_crucible(0, crate::direction::Direction::Right);
    let t = std::time::Instant::now();
    let path = c.heat_loss_at_target(map.len()-1, 1..3);
    assert_eq!(path.as_ref().unwrap().heat_loss_total(), 1008);
    println!("Part 1: {:?} = {:?}", path.unwrap().heat_loss_total(), t.elapsed());


    let mut c = map.get_crucible(0, crate::direction::Direction::Right);
    let t = std::time::Instant::now();
    let path = c.heat_loss_at_target(map.len()-1, 4..10);
    assert_eq!(path.as_ref().unwrap().heat_loss_total(), 1210);
    println!("Part 2: {:?} = {:?}", &path.unwrap().heat_loss_total(), t.elapsed());
}