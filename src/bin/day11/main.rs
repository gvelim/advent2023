mod universe;
mod galaxy;

use crate::universe::Universe;

fn main() {
    let run_part = |universe: &mut Universe, multiplier:usize| -> usize {
        universe.expand(multiplier);
        universe.clusters
            .iter()
            .enumerate()
            .map(|(i, from)| {
                universe.clusters
                    .iter()
                    .skip(i + 1)
                    .map(|to| from.distance_to(to))
                    .sum::<usize>()
            })
            .sum::<usize>()
    };
    let input = std::fs::read_to_string("src/bin/day11/input.txt").expect("Ops!");

    let mut universe = input.parse::<Universe>().expect("Failed to parse Universe!");
    let t = std::time::Instant::now();
    println!("Part 1 - Sum of shortest paths: {} - {:?}", run_part(&mut universe,2), t.elapsed());

    let mut universe = input.parse::<Universe>().expect("Failed to parse Universe!");
    let t = std::time::Instant::now();
    println!("Part 2 - Sum of shortest paths: {} - {:?}", run_part(&mut universe, 1_000_000), t.elapsed());
}
