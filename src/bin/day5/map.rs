use std::{ops::Range, str::FromStr, sync::Arc};
use super::mapping::*;

#[derive(Debug,Hash,Eq,PartialEq,Copy, Clone)]
pub(crate) enum MapType {
    Seed, Soil, Fertilizer, Water, Light, Temperature, Humidity, Location
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
pub(crate) struct Map {
    pub(crate) map: MapType,
    pub(crate) dest: MapType,
    pub(crate) mappings: Arc<[Mapping]>
}

impl Map {
    pub(crate) fn transform(&self, seed: u64) -> (u64,MapType) {
        self.mappings.iter()
            .filter_map(|mapping| mapping.transform(seed))
            .map(|seed| (seed, self.dest))
            .next()
            .unwrap_or( (seed, self.dest))
    }

    pub(crate) fn transform_range(&self, seeds: &[Range<u64>]) -> (Vec<Range<u64>>,MapType) {
        let mut queue: Vec<Range<u64>> = seeds.into();
        let mut out = vec![];

        for mapping in self.mappings.iter() {
            let mut tmp = vec![];
            while let Some(rng) = queue.pop() {
                let (rng, residual) = mapping.transform_range(&rng);
                rng.map(|r| out.push(r));
                tmp.extend(residual);
            }
            queue = tmp;
            // println!("{:?}",(mapping,&queue));
        }
        out.extend(queue);
        (out, self.dest)
    }
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
            mappings: maps
                .map(|m| m.parse::<Mapping>().expect("mapping::Ops"))
                .collect::<Arc<[_]>>()
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Seeds;

    #[test]
    fn test_map_transform_ranges() {
        let input = std::fs::read_to_string("./src/bin/day5/sample.txt").expect("Ops!");
        let mut split = input.split("\n\n");
        let seeds = split.next().unwrap().parse::<Seeds>().expect("Ops!");
        let map = split.next().unwrap().parse::<Map>().expect("Ops!");

        println!("{:?}",
            map.transform_range(&seeds.get_ranges())
        );
    }

    #[test]
    fn test_map_transform() {
        let input = std::fs::read_to_string("./src/bin/day5/sample.txt").expect("Ops!");
        let mut split = input.split("\n\n");
        let seeds = split.next().unwrap().parse::<Seeds>().expect("Ops!");
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
    fn test_parse_map() {
        let data = std::fs::read_to_string("./src/bin/day5/sample.txt").expect("Ops!");
        let input = data.split("\n\n").nth(1).unwrap();

        let map = input.parse::<Map>().expect("Map::Ops!");
        println!("{:?}",map);
        assert_eq!(
            map,
            Map { map: MapType::Seed, dest: MapType::Soil,
                    mappings: vec![
                        Mapping { src_base: 98..100, dst_base: 50 },
                        Mapping { src_base: 50..98, dst_base: 52 }
                    ].into()
                }
        )
    }
}
