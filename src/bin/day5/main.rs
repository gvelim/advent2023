mod map;
mod mapping;

use mapping::*;
use map::*;

fn main() {

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_seeds() {
        let seeds = INPUT.split("\n\n").next().unwrap();

        let seeds = seeds.split(':')
            .last().unwrap()
            .split_whitespace()
            .map(|num| u32::from_str_radix(num.trim(),10).expect("Seeds:Ops!"))
            .collect::<Vec<_>>();

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