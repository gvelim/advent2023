use crate::citymap::CityMap;
use crate::direction::Direction as D;

mod citymap;
mod direction;
mod crucible;
mod block;
mod path;

fn main() {
    let input = std::fs::read_to_string("src/bin/day17/input.txt").expect("File Not Found!");
    let map = input.parse::<CityMap>().expect("ops");

    let mut c = map.get_crucible(0, D::Right);
    let t = std::time::Instant::now();
    let path = c.find_path_to(map.len()-1, 1..3);
    assert_eq!(path.as_ref().unwrap().total_heat_loss(), 1008);
    println!("Part 1: {:?} = {:?}", path.unwrap().total_heat_loss(), t.elapsed());


    let mut c = map.get_crucible(0, D::Right);
    let t = std::time::Instant::now();
    let path = c.find_path_to(map.len()-1, 4..10);
    assert_eq!(path.as_ref().unwrap().total_heat_loss(), 1210);
    println!("Part 2: {:?} = {:?}", &path.unwrap().total_heat_loss(), t.elapsed());
}