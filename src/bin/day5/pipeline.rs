use std::{collections::HashMap, ops::Range, str::FromStr, sync::Arc};

use super::map::*;


pub(crate) struct Seeds(Arc<[u64]>);

impl Seeds {
    pub(crate) fn get_ranges(&self) -> Arc<[Range<u64>]>{
        self.0.chunks(2)
            .map(|r| (r[0]..r[0]+r[1]))
            .collect::<Arc<[_]>>()
    }

    #[inline]
    pub(crate) fn iter(&self) -> impl Iterator<Item=&u64> {
        self.0.iter()
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
            .map(|num| num.trim().parse::<u64>().expect("Seeds:Ops!"))
            .collect::<Arc<[_]>>()
        ))
    }
}

pub(crate) struct Pipeline {
    maps: HashMap<MapType,Map>
}

impl Pipeline {
    pub(crate) fn run(&self, start: (u64, MapType)) -> u64 {
        let (mut out, mut next) = start;

        while let Some(map) = self.maps.get(&next) {
            // print!("{:?}->",(out,next));
             (out, next) = map.transform(out);
        }
        // println!();
        out
    }
    pub(crate) fn run_ranges(&self, start: (&[Range<u64>],MapType)) -> Vec<Range<u64>> {
        let mut out: Vec<Range<u64>> = start.0.into();
        let mut next = start.1;

        println!();
        while let Some(map) = self.maps.get(&next) {
            // println!("{:?}->",(&out,next));
             (out, next) = map.transform_range(&out);
        }
        out
    }
}

impl FromStr for Pipeline {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split = input.split("\n\n").skip(1);
        Ok(
            Pipeline {
                maps: split
                    .into_iter()
                    .map(|map| map.parse::<Map>().expect("Ops!"))
                    .map(|map| (map.map, map))
                    .collect::<HashMap<MapType,Map>>()
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
        let seeds = input.parse::<Seeds>().expect("Ops!");
        let pipeline = input.parse::<Pipeline>().expect("Ops!");

        let min = pipeline.run_ranges((&seeds.get_ranges(),MapType::Seed))
            .into_iter()
            .min_by_key(|d| d.start )
            .unwrap();

        println!("{:?}",min);
        assert_eq!(min.start,46);
    }

    #[test]
    fn test_ranges_min_location() {
        let input = std::fs::read_to_string("./src/bin/day5/sample.txt").expect("Ops!");
        let seeds = input.parse::<Seeds>().expect("Ops!");
        let pipeline = input.parse::<Pipeline>().expect("Ops!");

        let min = seeds
            .get_ranges()
            .into_iter()
            .inspect(|range| println!("{:?}",range))
            .map(|range| {
                range.clone().map(|seed|
                    pipeline.run((seed,MapType::Seed))
                )
                .min()
                .unwrap_or(u64::MAX)
            })
            .min();

        assert_eq!(min, Some(46))
    }

    #[test]
    fn test_min_location() {
        let input = std::fs::read_to_string("./src/bin/day5/sample.txt").expect("Ops!");
        let seeds = input.parse::<Seeds>().expect("Ops!");
        let pipeline = input.parse::<Pipeline>().expect("Ops!");

        let min = seeds.0.iter()
            .map(|&seed|
                pipeline.run((seed,MapType::Seed))
            )
            .min();

        assert_eq!( Some(35),min);
    }

    #[test]
    fn test_pipeline() {
        let input = std::fs::read_to_string("./src/bin/day5/sample.txt").expect("Ops!");
        let seeds = input.parse::<Seeds>().expect("Ops!");
        let pipeline = input.parse::<Pipeline>().expect("Ops!");

        assert_eq!(82, pipeline.run((seeds.0[0], MapType::Seed)));
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
