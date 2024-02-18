use crate::citymap::CityMap;

mod citymap;
mod direction;
mod crucible;
pub(crate) mod block;

fn main() {
    let input = std::fs::read_to_string("src/bin/day17/input.txt").expect("File Not Found!");
    let map = input.parse::<CityMap>().expect("ops");

    let mut c = map.get_crucible(0, crate::direction::Direction::Right);
    let t = std::time::Instant::now();
    let loss = c.heat_loss_at_target(map.len()-1, 1..3);
    println!("Part 1: {:?} = {:?}", loss, t.elapsed());
    assert_eq!(loss, Some(102));

    let mut c = map.get_crucible(0, crate::direction::Direction::Right);
    let t = std::time::Instant::now();
    let loss = c.heat_loss_at_target(map.len()-1, 4..10);
    println!("Part 2: {:?} = {:?}", loss, t.elapsed());
    assert_eq!(loss, Some(1210));
}