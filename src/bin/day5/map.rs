use std::{ops::Range, rc::Rc, str::FromStr};
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
    map: MapType,
    dest: MapType,
    mappings: Rc<[Mapping]>
}

impl Map {
    pub(crate) fn id(&self) -> MapType {
        self.map
    }
}

pub trait Transform<T> {
    fn transform(&self, seed: T) -> (T,MapType) where T: Clone;
}

impl Transform<u64> for Map {
    fn transform(&self, seed: u64) -> (u64,MapType) where u64: Clone {
        self.mappings.iter()
            .filter_map(|mapping| mapping.transform(seed))
            .map(|seed| (seed, self.dest))
            .next()
            .unwrap_or( (seed, self.dest))
    }
}

impl Transform<Rc<[Range<u64>]>> for Map {
    fn transform(&self, seeds: Rc<[Range<u64>]>) -> (Rc<[Range<u64>]>,MapType) {
        let mut queue1: Vec<Range<u64>> = seeds.as_ref().into();
        let mut queue2 = Vec::with_capacity(seeds.len()*2);
        let mut out = Vec::with_capacity(seeds.len());

        for mapping in self.mappings.iter() {
            while let Some(rng) = queue1.pop() {
                // map input range into mapped and residual range(s)
                let (mapped, residual) = mapping.transform_range(&rng);
                // push mapped range to the output
                if let Some(r) = mapped { out.push(r) };
                // push residual to the queue for processing by subsequent mappings
                match residual {
                    RangeResidue::Single(a) => queue2.push(a),
                    RangeResidue::Double(a, b) => queue2.extend([a,b]),
                    _ => (),
                }
            }
            // flip/flop the pointers to the queues' memory allocation:
            // one is now empty and the other has the ranges for processing by the next mapping
            // so we avoid temporary vector and subsequenly heap allocation
            std::mem::swap::<Vec<Range<u64>>>(&mut queue1, &mut queue2);
            // println!("{:?}",(self.map, mapping,&queue1));
        }
        // add remaining residual ranges following the processing of all mappings
        queue1.extend(out);

        (queue1.into(), self.dest)
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
                .collect::<Rc<[_]>>()
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
            map.transform(seeds.get_ranges())
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
