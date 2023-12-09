mod map;
mod mapping;
mod pipeline;

use std::{ops::Range, str::FromStr, time::Instant};
use map::*;
use pipeline::*;
use rayon::prelude::*;
fn main() {
    let input = std::fs::read_to_string("src/bin/day5/input.txt").expect("Ops!");
    let seeds = input.parse::<Seeds>().expect("Ops!");
    let pipeline = input.parse::<Pipeline>().expect("Ops!");

    let t = Instant::now();
    let min = seeds.0.iter()
        .map(|&seed|
            pipeline.run((seed,MapType::Seed))
        )
        .min();

    println!("Part 1, min: {:?} - {:?}",min, t.elapsed());

    let t = Instant::now();
    let min = seeds
        .into_ranges()
        .into_par_iter()
        .inspect(|range| print!("{:?} - ",range))
        .map(|range| {
            range.map(|seed|
                    pipeline.run((seed,MapType::Seed))
                )
                .min()
                .unwrap()
        })
        .inspect(|_| println!("{:?}", t.elapsed()))
        .min();

    println!("Part 2, min: {:?}",min);
}

struct Seeds(Vec<u64>);

impl Seeds {
    fn into_ranges(&self) -> Vec<Range<u64>>{
        self.0.chunks(2)
            .map(|r| (r[0]..r[0]+r[1]))
            .collect::<Vec<_>>()
    }
}

impl FromStr for Seeds {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Seeds(
            input.split("\n\n")
            .next().unwrap()
            .split(':')
            .last().unwrap()
            .split_whitespace()
            .map(|num| u64::from_str_radix(num.trim(),10).expect("Seeds:Ops!"))
            .collect::<Vec<_>>()
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ranges_min_location() {
        let seeds = INPUT.split("\n\n").next().unwrap().parse::<Seeds>().expect("Ops!");
        let pipeline = INPUT.parse::<Pipeline>().expect("Ops!");

        let min = seeds
            .into_ranges()
            .into_iter()
            .inspect(|range| println!("{:?}",range))
            .map(|range| {
                range.map(|seed|
                    pipeline.run((seed,MapType::Seed))
                )
                    .min()
                    .unwrap()
            })
            .min();

        assert_eq!(min, Some(46))
    }
    #[test]
    fn test_ranges() {
        let seeds = INPUT.split("\n\n").next().unwrap().parse::<Seeds>().expect("Ops!");
        let ranges = seeds.into_ranges();
        assert_eq!(
            ranges,
            [79..93, 55..68]
        )
    }
    #[test]
    fn test_min_location() {
        let seeds = INPUT.split("\n\n").next().unwrap().parse::<Seeds>().expect("Ops!");
        let pipeline = INPUT.parse::<Pipeline>().expect("Ops!");

        let min = seeds.0.iter()
            .map(|&seed|
                pipeline.run((seed,MapType::Seed))
            )
            .min();

        assert_eq!( Some(35),min);
    }
    #[test]
    fn test_pipeline() {
        let seeds = INPUT.split("\n\n").next().unwrap().parse::<Seeds>().expect("Ops!");
        let pipeline = INPUT.parse::<Pipeline>().expect("Ops!");

        assert_eq!(
            82,
            pipeline.run( (seeds.0[0], MapType::Seed)))
        ;
    }
    #[test]
    fn test_map_transform() {
        let mut split = INPUT.split("\n\n");
        let seeds = split.next().unwrap().parse::<Seeds>().expect("Ops!");
        let map = split.next().unwrap().parse::<Map>().expect("Ops!");

        seeds.0.iter()
            .inspect(|seed| print!("Input: {seed}"))
            .map(|&seed| map.transform(seed))
            .for_each(|d| println!(" -> {:?}",d));

        assert_eq!(
            seeds.0.iter().map(|&seed| map.transform(seed)).collect::<Vec<_>>(),
            [(81, MapType::Soil), (14, MapType::Soil), (57, MapType::Soil),(13, MapType::Soil)]
        )
    }
    #[test]
    fn test_parse_seeds() {
        let mut split = INPUT.split("\n\n");
        let seeds = split.next().unwrap().parse::<Seeds>().expect("Ops!");
        assert_eq!(seeds.0,[79_u64,14,55,13]);
    }
    #[test]
    fn test_parse_map() {
        let input = INPUT.split("\n\n").skip(1).next().unwrap();

        println!("{:?}",input.parse::<Map>().expect("Map::Ops!"));

        assert_eq!(
            input.parse::<Map>().expect("Map::Ops!"),
            Map { map: MapType::Seed, dest: MapType::Soil,
                    mappings: vec![
                        Mapping { src_base: 98..100, dst_base: 50, len: 2 },
                        Mapping { src_base: 50..98, dst_base: 52, len: 48 }
                    ]
                }
        )
    }

    static INPUT: &str = 
            "seeds: 79 14 55 13\n\
            \n\
            seed-to-soil map:\n\
            50 98 2\n\
            52 50 48\n\
            \n\
            soil-to-fertilizer map:\n\
            0 15 37\n\
            37 52 2\n\
            39 0 15\n\
            \n\
            fertilizer-to-water map:\n\
            49 53 8\n\
            0 11 42\n\
            42 0 7\n\
            57 7 4\n\
            \n\
            water-to-light map:\n\
            88 18 7\n\
            18 25 70\n\
            \n\
            light-to-temperature map:\n\
            45 77 23\n\
            81 45 19\n\
            68 64 13\n\
            \n\
            temperature-to-humidity map:\n\
            0 69 1\n\
            1 0 69\n\
            \n\
            humidity-to-location map:\n\
            60 56 37\n\
            56 93 4";
}