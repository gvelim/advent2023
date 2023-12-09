mod map;
mod mapping;
mod pipeline;

use mapping::*;
use map::*;
use pipeline::*;

fn main() {
    let input = std::fs::read_to_string("src/bin/day5/input.txt").expect("Ops!");
    let seeds = Seeds::parse(input.as_str());
    let pipeline = input.parse::<Pipeline>().expect("Ops!");

    let min = seeds.iter()
        .map(|&seed|
            pipeline.run((seed,MapType::Seed))
        )
        .min();

    println!("Part 1, min: {:?}",min);
}

struct Seeds;
impl Seeds {
    fn parse(input: &str) -> Vec<u64> {
        input.split("\n\n")
            .next().unwrap()
            .split(':')
            .last().unwrap()
            .split_whitespace()
            .map(|num| u64::from_str_radix(num.trim(),10).expect("Seeds:Ops!"))
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_location() {
        let seeds = Seeds::parse(INPUT.split("\n\n").next().unwrap());
        let pipeline = INPUT.parse::<Pipeline>().expect("Ops!");

        let min = seeds.iter()
            .map(|&seed|
                pipeline.run((seed,MapType::Seed))
            )
            .min();

        assert_eq!( Some(35),min);
    }
    #[test]
    fn test_pipeline() {
        let seeds = Seeds::parse(INPUT.split("\n\n").next().unwrap());
        let pipeline = INPUT.parse::<Pipeline>().expect("Ops!");

        assert_eq!(
            82,
            pipeline.run( (seeds[0], MapType::Seed)))
        ;
    }
    #[test]
    fn test_map_transform() {
        let mut split = INPUT.split("\n\n");
        let seeds = Seeds::parse(split.next().unwrap());
        let map = split.next().unwrap().parse::<Map>().expect("Ops!");

        seeds.iter()
            .inspect(|seed| print!("Input: {seed}"))
            .map(|&seed| map.transform(seed))
            .for_each(|d| println!(" -> {:?}",d));

        assert_eq!(
            seeds.iter().map(|&seed| map.transform(seed)).collect::<Vec<_>>(),
            [(81, MapType::Soil), (14, MapType::Soil), (57, MapType::Soil),(13, MapType::Soil)]
        )
    }
    #[test]
    fn test_parse_seeds() {
        let mut split = INPUT.split("\n\n");
        let seeds = Seeds::parse(split.next().unwrap());
        assert_eq!(seeds,[79,14,55,13_u32]);
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