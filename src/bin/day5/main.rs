mod map;
mod mapping;
mod pipeline;

use std::time::Instant;
use map::MapType;
use pipeline::{ Run, Pipeline, Seeds};

fn main() -> Result<(),()> {
    let input = std::fs::read_to_string("src/bin/day5/input.txt").expect("Ops!");
    let seeds = input.parse::<Seeds>()?;
    let pipeline = input.parse::<Pipeline>()?;

    let t = Instant::now();
    let min = seeds.iter()
        .map(|&seed|
            pipeline.run(seed, MapType::Seed)
        )
        .min();

    println!("Part 1, min: {:?} - {:?}",min, t.elapsed());
    assert_eq!(min, Some(388_071_289));

    let t = Instant::now();
    let ranges = pipeline.run(seeds.get_ranges(), MapType::Seed);
    let min = ranges
        .into_iter()
        .min_by_key(|r| r.start)
        .unwrap();

    println!("Part 2, min: {:?} - {:?}",min.start, t.elapsed());
    assert_eq!(min.start, 84_206_669);

    Ok(())
}
