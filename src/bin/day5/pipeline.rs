use std::{collections::HashMap, num::ParseIntError, ops::Range, rc::Rc, str::FromStr};
use crate::map::MapError;

use super::map::{Transform,MapType,Map};

pub(crate) struct Seeds(Rc<[u64]>);

impl Seeds {
    pub(crate) fn get_ranges(&self) -> Rc<[Range<u64>]> {
        self.0.chunks(2)
            .map(|r| (r[0]..r[0]+r[1]))
            .collect::<Rc<[_]>>()
    }

    #[inline]
    pub(crate) fn iter(&self) -> impl Iterator<Item=&u64> {
        self.0.iter()
    }
}

impl FromStr for Seeds {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Seeds(
            input.split("\n\n")
            .next().unwrap()
            .split(':')
            .last().unwrap()
            .split_whitespace()
            .map(|num| num.trim().parse::<u64>())
            .collect::<Result<Rc<[_]>,ParseIntError>>()?
        ))
    }
}

pub(crate) struct Pipeline {
    maps: HashMap<MapType,Map>
}

pub(crate) trait Run<T> {
    fn run(&self, seed: T, map_type: MapType) -> T;
}

impl Run<u64> for Pipeline {
    fn run(&self, seed: u64, mut map_type: MapType) -> u64 {
        let mut out = seed;

        while let Some(map) = self.maps.get(&map_type) {
             (out, map_type) = map.transform(out);
        }
        out
    }
}

impl Run<Rc<[Range<u64>]>> for Pipeline {
    fn run(&self, seeds: Rc<[Range<u64>]>, mut map_type: MapType) -> Rc<[Range<u64>]> {
        let mut out = seeds.clone();

        while let Some(map) = self.maps.get(&map_type) {
             (out, map_type) = map.transform(out);
        }
        out
    }
}

impl FromStr for Pipeline {
    type Err = MapError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split = input.split("\n\n").skip(1);
        Ok(
            Pipeline {
                maps: split
                    .into_iter()
                    .map(|m| m.parse::<Map>())
                    .map(|m|
                        m.map(|map| (map.id(), map))
                    )
                    .collect::<Result<HashMap<MapType,Map>,MapError>>()?
            }
        )
    }
}


#[cfg(test)]
mod test_pipeline {
    use super::*;

    #[test]
    fn test_pipeline_ranges() {
        let input = std::fs::read_to_string("./src/bin/day5/sample.txt").expect("Ops!");
        let seeds = input.parse::<Seeds>().map_err(|e| panic!("{e}")).unwrap();
        let pipeline = input.parse::<Pipeline>().map_err(|e| panic!("{e:?}")).unwrap();

        let ranges = pipeline.run(seeds.get_ranges(), MapType::Seed);
        let min = ranges
            .iter()
            .min_by_key(|d| d.start )
            .unwrap();

        println!("{:?}",min);
        assert_eq!(min.start,46);
    }

    #[test]
    fn test_min_location() {
        let input = std::fs::read_to_string("./src/bin/day5/sample.txt").expect("Ops!");
        let seeds = input.parse::<Seeds>().expect("Ops!");
        let pipeline = input.parse::<Pipeline>().expect("Ops!");

        let min = seeds.0.iter()
            .map(|&seed|
                pipeline.run(seed, MapType::Seed)
            )
            .min();

        assert_eq!( Some(35),min);
    }

    #[test]
    fn test_pipeline() {
        let input = std::fs::read_to_string("./src/bin/day5/sample.txt").expect("Ops!");
        let seeds = input.parse::<Seeds>().expect("Ops!");
        let pipeline = input.parse::<Pipeline>().expect("Ops!");

        assert_eq!(82, pipeline.run(seeds.0[0], MapType::Seed));
    }
}

#[cfg(test)]
mod test_seeds {
    use super::*;

    #[test]
    fn test_ranges() {
        let input = std::fs::read_to_string("./src/bin/day5/sample.txt").expect("Ops!");
        let seeds = input.parse::<Seeds>().expect("Ops!");
        let ranges = seeds.get_ranges();
        assert_eq!(
            ranges,
            [79..93, 55..68].into()
        )
    }

    #[test]
    fn test_parse_seeds() {
        let input = std::fs::read_to_string("./src/bin/day5/sample.txt").expect("Ops!");
        let seeds = input.parse::<Seeds>().expect("Ops!");
        assert_eq!(seeds.0,[79_u64,14,55,13].into());
    }
}
