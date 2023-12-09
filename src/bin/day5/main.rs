use std::{ops::Range, str::FromStr};

fn main() {

}

#[derive(Debug,PartialEq)]
enum MapType {
    Seed, Soil, Fertilizer, Water, Light, Temperature, Humidity, Location
}
impl MapType {
}
impl FromStr for MapType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seed" => Ok(MapType::Seed),
            "soil" => Ok(MapType::Soil),
            "fertilizer" => Ok(MapType::Fertilizer),
            "water" => Ok(MapType::Water),
            "light" => Ok(MapType::Light),
            "temperature" => Ok(MapType::Temperature),
            "humidity" => Ok(MapType::Humidity),
            "location" => Ok(MapType::Location),
            _ => Err(format!("Cannot convert to MapType {s}"))
        }
    }
}

#[derive(Debug,PartialEq)]
struct Mapping {
    src_base: Range<u32>, // 98 (98,99)
    dst_base: u32, // 52
    len: u32
}

#[derive(Debug,PartialEq)]
struct Map {
    map: MapType,
    dest: MapType,
    mappings: Vec<Mapping>
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut maps = s.split("\n\n").next().unwrap().lines();

        let mut map_type = maps
            .next().unwrap()
            .split_whitespace()
            .next().unwrap()
            .split("-to-")
            .map(|map| map.parse::<MapType>().expect("map_type::Ops!"));

        Ok(Map {
            map: map_type.next().unwrap(), 
            dest: map_type.next().unwrap(),
            mappings: maps.map(|m| {
                let mut nums = m.split_whitespace();
                let dst_base = u32::from_str(nums.next().unwrap()).expect("dst_base::Ops!"); 
                let src_base = u32::from_str(nums.next().unwrap()).expect("src_base::Ops!"); 
                let len = u32::from_str(nums.next().unwrap()).expect("len::Ops!");

                Mapping { 
                    dst_base, 
                    src_base: (src_base..src_base + len), 
                    len
                }
                })
                .collect::<Vec<_>>()
        })
    }
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