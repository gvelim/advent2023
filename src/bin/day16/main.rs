mod direction;
mod cavern;

use crate::cavern::{Cavern,entry_points};
use crate::direction::Direction as D;

fn main() {
    let inp = std::fs::read_to_string("src/bin/day16/input.txt").expect("Ops!");
    let mut cavern = inp.parse::<Cavern>().unwrap();

    let t = std::time::Instant::now();
    cavern.move_beam(0, D::Right);
    println!("Part 1 : Cavern Energy = {:?} - {:?}", cavern.measure_energy(), t.elapsed());
    assert_eq!(cavern.measure_energy(), 6902);

    let t = std::time::Instant::now();
    let m = entry_points(cavern.width, cavern.lines)
            .map(|(idx,dir)| {
                cavern.energise(idx,dir);
                cavern.measure_energy()
            })
            .max();
    println!("Part 2 : Max Energy = {:?} - {:?}", m, t.elapsed());
    assert_eq!(m,Some(7697));
}


