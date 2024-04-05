mod map;
mod mapping;
mod pipeline;

use std::time::Instant;
use map::*;
use pipeline::*;
use rayon::prelude::*;

fn main() {
    let input = std::fs::read_to_string("src/bin/day5/input.txt").expect("Ops!");
    let seeds = input.parse::<Seeds>().expect("Ops!");
    let pipeline = input.parse::<Pipeline>().expect("Ops!");

    let t = Instant::now();
    let min = seeds.iter()
        .map(|&seed|
            pipeline.run((seed,MapType::Seed))
        )
        .min();

    println!("Part 1, min: {:?} - {:?}",min, t.elapsed());
    assert_eq!(min, Some(388_071_289));

    let t = Instant::now();
    let min = seeds
        .get_ranges()
        .into_par_iter()
        .inspect(|range| print!("{:?} - ",range))
        .map(|range| {
            range.clone().map(|seed|
                    pipeline.run((seed,MapType::Seed))
                )
                .min()
                .unwrap_or(u64::MAX)
        })
        .inspect(|_| println!("{:?}", t.elapsed()))
        .min();

    println!("Part 2, min: {:?}",min);
    assert_eq!(min, Some(84_206_669));
}
